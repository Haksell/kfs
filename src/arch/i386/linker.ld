ENTRY(start)

SECTIONS {
    . = 1m;

    .boot :
    {
        *(.multiboot_header)
    }

    .text :
    {
        *(.text)
    }
}