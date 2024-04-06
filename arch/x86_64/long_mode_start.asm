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
    ; we shouldn't arrive here
    mov rax, 0x2f212f4d2f532f41
    mov qword [0xb8000], rax
    hlt