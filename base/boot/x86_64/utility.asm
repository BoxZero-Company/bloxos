; BloxOS x86-64 assembly utility.
; System V AMD64 ABI: arguments arrive in RDI and RSI.
; This function returns their sum in RAX.
; This is a test routine, NOT a bootloader.

BITS 64
DEFAULT REL

SECTION .text
GLOBAL bloxos_asm_add

bloxos_asm_add:
    mov rax, rdi
    add rax, rsi
    ret
