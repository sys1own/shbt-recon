//! Zero-copy POSIX shared-memory SPSC telemetry ring —
//! `SharedMemoryTransport` / `TelemetryRing` transferred from
//! `sys1own/shbt-cf` (`shbt-fabrication-hil/src/{shm,ring,telemetry}.rs`).
//!
//! `shm_open` + `ftruncate` + `mmap` layout: a `RingHeader` on the first
//! 64-byte cache line followed by `#[repr(C, align(64))]` `TelemetryFrame`s.

use std::io;
use std::sync::atomic::{AtomicU32, Ordering};

/// Ring magic identifying an initialized telemetry segment.
pub const RING_MAGIC: u32 = 0x5348_4254; // "SHBT"
/// Cache line / frame alignment.
pub const CACHE_LINE: usize = 64;

/// Zero-copy telemetry frame — one 64-byte cache line.
#[repr(C, align(64))]
#[derive(Clone, Copy, Debug, Default)]
pub struct TelemetryFrame {
    pub sequence_id: u64,
    pub timestamp_fs: u64,
    pub metric_det: f64,
    pub gram_lambda_min: f64,
    pub kapitza_temp_mk: f32,
    pub power_output_kw: f32,
    pub status_flags: u32,
    pub reserved_padding: [u8; 12],
    pub frame_crc32: u32,
}

const _: () = assert!(std::mem::size_of::<TelemetryFrame>() == 64);
const _: () = assert!(std::mem::align_of::<TelemetryFrame>() == 64);

/// SPSC ring header on the first cache line of the shared region.
#[repr(C, align(64))]
pub struct RingHeader {
    pub magic: u32,
    pub capacity: u32,
    pub head: AtomicU32,
    pub tail: AtomicU32,
    pub dropped: AtomicU32,
    pub _pad: [u8; 40],
}

const _: () = assert!(std::mem::size_of::<RingHeader>() == 64);

/// Total byte length of a region with `capacity` frames.
pub fn region_len(capacity: usize) -> usize {
    CACHE_LINE + capacity * std::mem::size_of::<TelemetryFrame>()
}

/// CRC-32 (IEEE) of a frame, used as `frame_crc32`.
pub fn frame_crc32(frame: &TelemetryFrame) -> u32 {
    let bytes = unsafe {
        std::slice::from_raw_parts(
            (frame as *const TelemetryFrame).cast::<u8>(),
            std::mem::size_of::<TelemetryFrame>(),
        )
    };
    let mut crc = 0xFFFF_FFFFu32;
    // CRC field lives at offset 0x38; cover bytes 0..56 only.
    for &b in &bytes[..56] {
        crc ^= b as u32;
        for _ in 0..8 {
            crc = (crc >> 1) ^ (0xEDB8_8320 & (0u32.wrapping_sub(crc & 1)));
        }
    }
    !crc
}

/// Bounds-checked SPSC ring view over any 64-aligned byte region
/// (shared memory or a local buffer in tests).
pub struct SpscRing {
    header: *mut RingHeader,
    frames: *mut TelemetryFrame,
    capacity: usize,
}

unsafe impl Send for SpscRing {}

impl SpscRing {
    /// Initialize a fresh ring over `region` (must be ≥ region_len(capacity)).
    ///
    /// # Safety
    /// `region` must point to a writable, 64-byte aligned region that lives
    /// for the lifetime of the returned ring.
    pub unsafe fn init(region: *mut u8, capacity: usize) -> Self {
        std::ptr::write_bytes(region, 0, region_len(capacity));
        let header = region.cast::<RingHeader>();
        (*header).magic = RING_MAGIC;
        (*header).capacity = capacity as u32;
        Self {
            header,
            frames: region.add(CACHE_LINE).cast::<TelemetryFrame>(),
            capacity,
        }
    }

    /// Attach to an already-initialized region.
    ///
    /// # Safety
    /// Same contract as [`Self::init`]; the region must hold a valid header.
    pub unsafe fn attach(region: *mut u8) -> io::Result<Self> {
        let header = region.cast::<RingHeader>();
        if (*header).magic != RING_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bad ring magic"));
        }
        let capacity = (*header).capacity as usize;
        Ok(Self {
            header,
            frames: region.add(CACHE_LINE).cast::<TelemetryFrame>(),
            capacity,
        })
    }

    /// Producer: push a frame (computes CRC); false when full.
    pub fn push(&self, mut frame: TelemetryFrame) -> bool {
        let hdr = unsafe { &*self.header };
        let head = hdr.head.load(Ordering::Acquire);
        let tail = hdr.tail.load(Ordering::Acquire);
        if head.wrapping_sub(tail) as usize >= self.capacity {
            hdr.dropped.fetch_add(1, Ordering::Relaxed);
            return false;
        }
        frame.frame_crc32 = frame_crc32(&frame);
        unsafe {
            std::ptr::write_volatile(self.frames.add((head as usize) % self.capacity), frame);
        }
        hdr.head.store(head.wrapping_add(1), Ordering::Release);
        true
    }

    /// Consumer: pop the next frame; `None` when empty.
    pub fn pop(&self) -> Option<TelemetryFrame> {
        let hdr = unsafe { &*self.header };
        let tail = hdr.tail.load(Ordering::Acquire);
        let head = hdr.head.load(Ordering::Acquire);
        if tail == head {
            return None;
        }
        let frame =
            unsafe { std::ptr::read_volatile(self.frames.add((tail as usize) % self.capacity)) };
        hdr.tail.store(tail.wrapping_add(1), Ordering::Release);
        Some(frame)
    }

    /// Number of frames available to the consumer.
    pub fn len(&self) -> usize {
        let hdr = unsafe { &*self.header };
        hdr.head
            .load(Ordering::Acquire)
            .wrapping_sub(hdr.tail.load(Ordering::Acquire)) as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(unix)]
pub mod posix {
    //! POSIX `shm_open`/`mmap` transport (from shbt-cf `shm.rs`).
    use super::{region_len, RingHeader, SpscRing, RING_MAGIC};
    use std::ffi::CString;
    use std::io;
    use std::ptr::{self, NonNull};

    /// A `mmap`ed POSIX shared-memory segment.
    #[derive(Debug)]
    pub struct SharedMemory {
        ptr: NonNull<u8>,
        len: usize,
        name: Option<CString>,
        owner: bool,
    }

    fn c_name(name: &str) -> io::Result<CString> {
        let n = if name.starts_with('/') {
            name.to_string()
        } else {
            format!("/{name}")
        };
        CString::new(n).map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "bad shm name"))
    }

    impl SharedMemory {
        /// Creates (or replaces) segment `name` sized for `capacity` frames.
        pub fn create(name: &str, capacity: usize) -> io::Result<Self> {
            let len = region_len(capacity);
            let cname = c_name(name)?;
            unsafe {
                libc::shm_unlink(cname.as_ptr());
                let fd = libc::shm_open(cname.as_ptr(), libc::O_RDWR | libc::O_CREAT, 0o600);
                if fd < 0 {
                    return Err(io::Error::last_os_error());
                }
                if libc::ftruncate(fd, len as libc::off_t) != 0 {
                    let e = io::Error::last_os_error();
                    libc::close(fd);
                    return Err(e);
                }
                let p = libc::mmap(
                    ptr::null_mut(),
                    len,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd,
                    0,
                );
                libc::close(fd);
                if p == libc::MAP_FAILED {
                    return Err(io::Error::last_os_error());
                }
                ptr::write_bytes(p, 0, len);
                let hdr = p as *mut RingHeader;
                (*hdr).capacity = capacity as u32;
                (*hdr).magic = RING_MAGIC;
                Ok(Self {
                    ptr: NonNull::new(p as *mut u8).unwrap(),
                    len,
                    name: Some(cname),
                    owner: true,
                })
            }
        }

        /// Attaches to an existing segment; polls ≤ ~1 s for header init.
        pub fn attach(name: &str) -> io::Result<Self> {
            let cname = c_name(name)?;
            unsafe {
                let fd = libc::shm_open(cname.as_ptr(), libc::O_RDWR, 0o600);
                if fd < 0 {
                    return Err(io::Error::last_os_error());
                }
                let mut len = 0usize;
                for _ in 0..1000 {
                    let mut st = std::mem::zeroed::<libc::stat>();
                    if libc::fstat(fd, &mut st) == 0 && st.st_size >= 64 {
                        len = st.st_size as usize;
                        break;
                    }
                    std::thread::yield_now();
                }
                if len == 0 {
                    libc::close(fd);
                    return Err(io::Error::new(
                        io::ErrorKind::TimedOut,
                        "shm not initialised",
                    ));
                }
                let p = libc::mmap(
                    ptr::null_mut(),
                    len,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd,
                    0,
                );
                libc::close(fd);
                if p == libc::MAP_FAILED {
                    return Err(io::Error::last_os_error());
                }
                Ok(Self {
                    ptr: NonNull::new(p as *mut u8).unwrap(),
                    len,
                    name: Some(cname),
                    owner: false,
                })
            }
        }

        /// Ring view over the mapped region (creator: after `create`).
        pub fn ring(&self) -> SpscRing {
            unsafe { SpscRing::attach(self.ptr.as_ptr()) }
                .expect("shm segment holds a valid ring header")
        }
    }

    impl Drop for SharedMemory {
        fn drop(&mut self) {
            unsafe {
                libc::munmap(self.ptr.as_ptr().cast(), self.len);
                if self.owner {
                    if let Some(name) = &self.name {
                        libc::shm_unlink(name.as_ptr());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_is_one_cache_line() {
        assert_eq!(std::mem::size_of::<TelemetryFrame>(), 64);
        assert_eq!(std::mem::align_of::<TelemetryFrame>(), 64);
    }

    #[test]
    fn spsc_ring_roundtrip() {
        let capacity = 8;
        let mut buf = vec![0u8; region_len(capacity) + 64];
        let base = unsafe {
            let p = buf.as_mut_ptr();
            p.add(p.align_offset(64))
        };
        let ring = unsafe { SpscRing::init(base, capacity) };
        for i in 0..capacity as u64 {
            let f = TelemetryFrame {
                sequence_id: i,
                metric_det: -1.0,
                gram_lambda_min: 0.5,
                ..TelemetryFrame::default()
            };
            assert!(ring.push(f), "push {i}");
        }
        assert!(!ring.push(TelemetryFrame::default())); // full
        for i in 0..capacity as u64 {
            let f = ring.pop().unwrap();
            assert_eq!(f.sequence_id, i);
            assert_eq!(f.frame_crc32, frame_crc32(&f));
        }
        assert!(ring.pop().is_none());
    }
}
