section .multiboot_header
header_start:
    dd 0xe85250d6 ; multiboot2 magic number
    dd 0 ; architecture: protected mode 1386
    dd header_end - header_start
    dd 0x100000000 - (0xe85250d6 + header_end - header_start) ; checksum
    dw 0 ; type (of what?)
    dw 0 ; flags (of what?)
    dd 8 ; size (of what?)
header_end: