global start
extern check_cpuid, check_multiboot, kernel_main

section .text
bits 32

start:
    mov esp, stack_top
    call check_multiboot
    call check_cpuid
    call kernel_main

section .bss
align 4096
stack_bottom:
    resb 4096 * 128
stack_top: