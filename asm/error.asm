bits 32
global error

section .data

hex_chars db "0123456789ABCDEF"

section .text

%macro WRITE_CHAR 2
    mov byte [%1], %2
    mov byte [%1 + 1], 0x4f
%endmacro

error:
    WRITE_CHAR 0xb8000, 'E'
    WRITE_CHAR 0xb8002, 'R'
    WRITE_CHAR 0xb8004, 'R'
    WRITE_CHAR 0xb8006, 'O'
    WRITE_CHAR 0xb8008, 'R'
    WRITE_CHAR 0xb800a, ':'
    WRITE_CHAR 0xb800c, ' '
    call print_hex
    hlt

print_hex:
    mov esi, 0xb800e
    mov ecx, 8

print_next_digit:
    mov ebx, eax
    shr ebx, 28
    and ebx, 0xF
    mov bl, [hex_chars + ebx]
    WRITE_CHAR esi, bl
    add esi, 2
    shl eax, 4
    loop print_next_digit
    ret