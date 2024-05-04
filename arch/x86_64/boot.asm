global start, gdt64.data
extern check_multiboot, error, long_mode_start

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    call check_long_mode
    call set_up_page_tables
    call enable_paging
    lgdt [gdt64.pointer]
    jmp gdt64.code:long_mode_start

; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
; in the FLAGS register. If we can flip it, CPUID is available.
check_cpuid:
    pushfd
    pop eax
    mov ecx, eax
    xor eax, 1 << 21
    push eax
    popfd
    pushfd
    pop eax
    push ecx
    popfd
    cmp eax, ecx
    je .no_cpuid
    ret
    .no_cpuid:
        mov al, '1'
        jmp error

check_long_mode:
    mov eax, 0x80000000
    cpuid
    cmp eax, 0x80000001
    jb .no_long_mode
    mov eax, 0x80000001
    cpuid
    test edx, 1 << 29
    jz .no_long_mode
    ret
    .no_long_mode:
        mov al, '2'
        jmp error

set_up_page_tables:
    ; P4[0] = P3
    mov eax, p3_table
    or eax, 0b11 ; present + writable
    mov [p4_table], eax
    ; P3[0] = P2
    mov eax, p2_table
    or eax, 0b11 ; present + writable
    mov [p3_table], eax
    ; map each P2 entry to a huge 2MiB page
    mov ecx, 0
    .map_p2_table:
        mov eax, 0x200000 ; 2MiB
        mul ecx
        or eax, 0b10000011 ; present + writable + huge
        mov [p2_table + ecx * 8], eax
        inc ecx
        cmp ecx, 512
        jne .map_p2_table
    ret

enable_paging:
    ; load P4 to cr3 register (cpu uses this to access the P4 table)
    mov eax, p4_table
    mov cr3, eax
    ; enable PAE-flag in cr4 (Physical Address Extension)
    mov eax, cr4
    or eax, 1 << 5
    mov cr4, eax
    ; set the long mode bit in the EFER MSR (model specific register)
    mov ecx, 0xC0000080
    rdmsr
    or eax, 1 << 8
    wrmsr
    ; enable paging in the cr0 register
    mov eax, cr0
    or eax, 1 << 16 ; write protect
    or eax, 1 << 31 ; paging
    mov cr0, eax
    ret

section .bss
align 4096
p4_table:
    resb 4096
p3_table:
    resb 4096
p2_table:
    resb 4096
stack_bottom:
    resb 4096 * 128
stack_top:

section .rodata
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment, (1<<41)?
.data: equ $ - gdt64
    dq (1<<41) | (1<<44) | (1<<47) ; data segment, (1<<53)?
.pointer:
    dw $ - gdt64 - 1
    dq gdt64
