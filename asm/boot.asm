global stack_bottom, stack_top, start
extern check_cpuid, check_multiboot, kernel_main

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    lgdt [gdt.pointer]
    mov ax, gdt.kernel_stack
    mov ss, ax
    mov ax, gdt.kernel_data
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    jmp gdt.kernel_code:kernel_main

section .bss
align 4096
stack_bottom:
    resb 4096 * 1024
stack_top:

section .rodata
align 8

%macro DEFINE_GDT_SEGMENT 1
    dw 0xFFFF     ; limit low
    dw 0x0000     ; base low
    db 0x00       ; base middle
    db %1         ; access
    db 0b11001111 ; granularity
    db 0x00       ; base high
%endmacro

gdt:
    dq 0
.kernel_code: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b10011011
.kernel_data: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b10010011
.kernel_stack: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b10010111
.user_code: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b11111011
.user_data: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b11110011
.user_stack: equ $ - gdt
    DEFINE_GDT_SEGMENT 0b11110111
.pointer:
    dw $ - gdt - 1
    dd gdt