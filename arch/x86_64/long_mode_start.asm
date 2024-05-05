global long_mode_start
extern gdt64.data, kernel_main

section .text
bits 64

long_mode_start:
    mov ax, gdt64.data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    call kernel_main
