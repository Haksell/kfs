ENTRY(start)

SECTIONS {
    . = 1m;

    .boot : {
        KEEP(*(.multiboot_header))
    }

    .text : {
        *(.text)
    }
}