global error

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
    WRITE_CHAR 0xb800e, al
    hlt