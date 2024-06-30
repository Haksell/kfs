global kernel_code, gdt_start, gdt_pointer, stack_bottom, stack_top, start
extern check_cpuid, check_multiboot, kernel_main, error


section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    call set_up_page_tables
    call enable_paging
    lgdt [gdt_pointer]
    call set_protected_mode ; obligatory ?

    jmp kernel_code:flush_cpu

set_protected_mode:
    mov eax, cr0
    or eax, 1
    mov cr0, eax
    ret

flush_cpu:
    mov ax, kernel_data
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    push ebx
    ; we have to push something on top, but why?
    push 0x69420

    jmp kernel_main

set_up_page_tables:
    ; map page_directory table recursively
    mov eax, page_directory
    or eax, 0b11 ; present + writable
    mov [page_directory + 1023 * 4], eax

    mov ecx, 0
    .map_page_directory:
        mov eax, 0x400000
        mul ecx
        or eax, 0b10000011 ; present + writable + huge
        mov [page_directory + ecx * 4], eax
        inc ecx
        cmp ecx, 1024
        jne .map_page_directory
    ret

enable_paging:
    ; load page_directory to cr3 register
    mov eax, page_directory
    mov cr3, eax
    ; enable PSE-flag in cr4 (Page Size Extension)
    mov eax, cr4
    or eax, 1 << 4
    mov cr4, eax
    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax
    ret


section .bss
align 4096
page_directory:
    resb 4096
stack_bottom:
    resb 4096 * 1024
stack_top:

section .rodata

%macro DEFINE_GDT_SEGMENT 1
    dw 0xFFFF     ; limit low
    dw 0x0000     ; base low
    db 0x00       ; base middle
    db %1         ; access
    db 0b11001111 ; granularity
    db 0x00       ; base high
%endmacro

gdt_start:
    dq 0
kernel_code: equ $ - gdt_start
    DEFINE_GDT_SEGMENT 0b10011011
kernel_data: equ $ - gdt_start
    DEFINE_GDT_SEGMENT 0b10010011
; user_code: equ $ - gdt_start
;     DEFINE_GDT_SEGMENT 0b11111011
; user_data: equ $ - gdt_start
;     DEFINE_GDT_SEGMENT 0b11110011
gdt_pointer:
    dw $ - gdt_start - 1
    dd gdt_start