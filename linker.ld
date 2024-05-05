ENTRY(start)

SECTIONS {
    . = 0x00000800;

    .rodata : {
        *(.rodata)
    }

    . = 1m;

    .boot : {
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text)
    }
}