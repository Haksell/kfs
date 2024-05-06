ENTRY(start)

SECTIONS {
    .rodata 0x00000800 : {
        *(.rodata)
    }

    .boot 1M : {
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text)
    }
}