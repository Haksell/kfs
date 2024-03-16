global start

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    ; call set_up_page_tables
    ; call enable_paging
    extern rust_main
    call rust_main
    hlt


check_multiboot:
    cmp eax, 0x36d76289
    jne .no_multiboot
    ret
    .no_multiboot:
        mov al, "0"
        jmp error


set_up_page_tables:
    ; P4[0] = P3
    mov eax, p3_table
    or eax, 0b11
    mov [p4_table], eax
    ; P3[0] = P2
    mov eax, p2_table
    or eax, 0b11
    mov [p3_table], eax
    ; map each P2 entry to a huge 2MiB page
    mov ecx, 0
    .map_p2_table:
        mov eax, 0x200000
        mul ecx
        or eax, 0b10000011 ; present + writable + huge
        mov [p2_table + ecx * 8], eax
        inc ecx
        cmp ecx, 512
        jne .map_p2_table
    ret


enable_paging:
    mov eax, p2_table
    mov cr3, eax

    mov eax, cr0
    or eax, 0x80000001
    mov cr0, eax

    ; Enable PSE ?
    ret


error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    mov byte [0xb800a], al
    hlt


section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 4
stack_top: