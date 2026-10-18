/*
 * linker.ld - GNU linker script for the SHBT-R bare-metal microkernel.
 *
 * Targets freestanding RISC-V/x86_64 environments.  RAM origin and length are
 * defaults and may be overridden by the build system with -Ttext/-Tdata-style
 * options or by providing an adjusted linker script.
 */

ENTRY(_start)

MEMORY
{
    /* Default RAM region: adjust ORIGIN/LENGTH per board/platform. */
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 0x00100000
}

SECTIONS
{
    . = ORIGIN(RAM);

    .text : {
        *(.text.init)
        *(.text .text.*)
        *(.gnu.linkonce.t.*)
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
        *(.gnu.linkonce.r.*)
    } > RAM

    .data : {
        __data_start = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        *(.gnu.linkonce.d.*)
        __data_end = .;
    } > RAM

    .bss (NOLOAD) : {
        __bss_start = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        *(.gnu.linkonce.b.*)
        __bss_end = .;
    } > RAM

    /*
     * UnifiedStinespringFrame arena: 2112 bytes, 64-byte aligned, not loaded
     * into the ELF file image.  Symbols mark the inclusive [start, end).
     */
    .stinespring_frame (NOLOAD) : ALIGN(64) {
        __stinespring_start = .;
        . = . + 2112;
        __stinespring_end = .;
    } > RAM

    /*
     * Stinespring 16-bank dark magazine arena: 33,792 B (16 x 2112 B), 64-byte aligned, NOLOAD.
     */
    .stinespring_magazine (NOLOAD) : ALIGN(64) {
        __stinespring_magazine_start = .;
        . = . + 33792;
        __stinespring_magazine_end = .;
    } > RAM

    __end = .;
}
