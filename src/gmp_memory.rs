//! Custom, deterministic memory allocator for the GMP/MPFR engine used by `rug`.
//!
//! A size-class free-list allocator services the small fixed-size allocations
//! that MPFR makes for high-precision limbs, keeping the hot path inside a
//! pre-resident arena and avoiding the libc heap.  Requests larger than the
//! largest pool class fall back to the system allocator.

use std::cmp;
use std::os::raw::c_void;
use std::ptr;
use std::sync::{Mutex, Once};

const ARENA_SIZE: usize = 16 * 1024 * 1024; // 16 MiB
const SIZE_CLASSES: [usize; 5] = [32, 64, 128, 256, 512];
const MAX_CLASS_SIZE: usize = SIZE_CLASSES[SIZE_CLASSES.len() - 1];
const NULL_OFFSET: usize = usize::MAX;

#[repr(C, align(16))]
struct Arena([u8; ARENA_SIZE]);

impl Arena {
    const fn new() -> Self {
        Arena([0; ARENA_SIZE])
    }
}

#[repr(C)]
struct FreeNode {
    next: usize,
}

struct Allocator {
    arena: Arena,
    bump: usize,
    free_lists: [usize; SIZE_CLASSES.len()],
}

impl Allocator {
    const fn new() -> Self {
        Allocator {
            arena: Arena::new(),
            bump: 0,
            free_lists: [NULL_OFFSET; SIZE_CLASSES.len()],
        }
    }

    fn in_arena(&self, ptr: *mut u8) -> bool {
        let start = self.arena.0.as_ptr() as usize;
        let p = ptr as usize;
        p >= start && p < start + ARENA_SIZE
    }

    unsafe fn alloc(&mut self, size: usize) -> *mut u8 {
        if size == 0 {
            return ptr::null_mut();
        }
        if size > MAX_CLASS_SIZE {
            let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
            return std::alloc::alloc(layout);
        }
        let class = size_class(size);
        let block_size = SIZE_CLASSES[class];

        if self.free_lists[class] != NULL_OFFSET {
            let node_offset = self.free_lists[class];
            let node_ptr = self.arena.0.as_mut_ptr().add(node_offset) as *mut FreeNode;
            self.free_lists[class] = (*node_ptr).next;
            return node_ptr as *mut u8;
        }

        if self.bump + block_size > ARENA_SIZE {
            let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
            return std::alloc::alloc(layout);
        }

        let ptr = self.arena.0.as_mut_ptr().add(self.bump);
        self.bump += block_size;
        ptr
    }

    unsafe fn free(&mut self, ptr: *mut u8, size: usize) {
        if ptr.is_null() || size == 0 {
            return;
        }
        if !self.in_arena(ptr) {
            let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
            std::alloc::dealloc(ptr, layout);
            return;
        }
        let class = size_class(size);
        let node_ptr = ptr as *mut FreeNode;
        (*node_ptr).next = self.free_lists[class];
        let offset = ptr.offset_from(self.arena.0.as_ptr()) as usize;
        self.free_lists[class] = offset;
    }

    unsafe fn realloc(&mut self, ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
        if ptr.is_null() {
            return self.alloc(new_size);
        }
        if new_size == 0 {
            self.free(ptr, old_size);
            return ptr::null_mut();
        }

        let old_class = if old_size <= MAX_CLASS_SIZE {
            size_class(old_size)
        } else {
            usize::MAX
        };
        let new_class = if new_size <= MAX_CLASS_SIZE {
            size_class(new_size)
        } else {
            usize::MAX
        };

        if self.in_arena(ptr) && old_class == new_class && old_class != usize::MAX {
            return ptr;
        }

        let new_ptr = self.alloc(new_size);
        if new_ptr.is_null() {
            return ptr::null_mut();
        }
        let copy_size = cmp::min(old_size, new_size);
        ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
        self.free(ptr, old_size);
        new_ptr
    }
}

fn size_class(size: usize) -> usize {
    for (i, &class) in SIZE_CLASSES.iter().enumerate() {
        if size <= class {
            return i;
        }
    }
    unreachable!()
}

static GMP_ALLOCATOR: Mutex<Allocator> = Mutex::new(Allocator::new());
static INIT: Once = Once::new();

/// Install the custom GMP memory functions.  Safe to call repeatedly.
pub fn init() {
    INIT.call_once(|| unsafe {
        gmp_mpfr_sys::gmp::set_memory_functions(
            Some(gmp_alloc),
            Some(gmp_realloc),
            Some(gmp_free),
        );
    });
}

extern "C" fn gmp_alloc(size: usize) -> *mut c_void {
    unsafe {
        if let Ok(mut guard) = GMP_ALLOCATOR.lock() {
            return guard.alloc(size) as *mut c_void;
        }
        let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
        std::alloc::alloc(layout) as *mut c_void
    }
}

unsafe extern "C" fn gmp_realloc(
    ptr: *mut c_void,
    old_size: usize,
    new_size: usize,
) -> *mut c_void {
    if let Ok(mut guard) = GMP_ALLOCATOR.lock() {
        return guard.realloc(ptr as *mut u8, old_size, new_size) as *mut c_void;
    }
    let new_layout = std::alloc::Layout::from_size_align(new_size, 16).unwrap();
    let new_ptr = std::alloc::alloc(new_layout);
    if !ptr.is_null() && !new_ptr.is_null() {
        let copy_size = cmp::min(old_size, new_size);
        ptr::copy_nonoverlapping(ptr as *const u8, new_ptr, copy_size);
        let old_layout = std::alloc::Layout::from_size_align(old_size, 16).unwrap();
        std::alloc::dealloc(ptr as *mut u8, old_layout);
    }
    new_ptr as *mut c_void
}

unsafe extern "C" fn gmp_free(ptr: *mut c_void, size: usize) {
    if ptr.is_null() {
        return;
    }
    if let Ok(mut guard) = GMP_ALLOCATOR.lock() {
        guard.free(ptr as *mut u8, size);
    } else {
        let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
        std::alloc::dealloc(ptr as *mut u8, layout);
    }
}
