global long_mode_start
extern kernel_main

section .text
bits 64

long_mode_start:
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    call kernel_main
    mov rax, 0x2f412f472f4f2f4f
    mov qword [0xb8000], rax
    mov qword [0xb8008], rax
    hlt
