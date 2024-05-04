global start
extern check_multiboot, kernel_main

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call kernel_main

; TODO: 3-level memory paging explained in kfs-3
section .bss
align 4096
stack_bottom:
    resb 4096 * 128
stack_top: