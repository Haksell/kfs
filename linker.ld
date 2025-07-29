ENTRY(start)

SECTIONS {
    .rodata 0x800 : {
        *(.rodata .rodata.*)
    }

    .boot 1M : {
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text .text.*)
    }

    .data.rel.ro : {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}