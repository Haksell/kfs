global long_mode_start
extern gdt64.data, kernel_main

section .text
bits 64

long_mode_start:
    mov ax, gdt64.data
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov ax, 0
    mov fs, ax
    mov gs, ax
    call kernel_main
