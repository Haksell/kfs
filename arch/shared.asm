global check_multiboot, error

section .text
bits 32

check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
    .no_multiboot:
        mov al, '0'
        jmp error

error:
    mov word [0xb8000], 0x4f45
    mov word [0xb8002], 0x4f52
    mov word [0xb8004], 0x4f52
    mov word [0xb8006], 0x4f4f
    mov word [0xb8008], 0x4f52
    mov word [0xb800a], 0x4f3a
    mov word [0xb800c], 0x4f20
    mov byte [0xb800e], al
    mov byte [0xb800f], 0x4f
    hlt
