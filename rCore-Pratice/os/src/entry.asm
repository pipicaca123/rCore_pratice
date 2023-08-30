    .section .text.entry # put infomation all into the section which name as .text.entry
    .globl _start

_start:
    la sp, boot_stack_top  # load address
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16

    .globl boot_stack_top
boot_stack_top: