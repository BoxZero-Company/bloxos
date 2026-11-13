; Load an IDT descriptor supplied by the kernel (System V AMD64 ABI).
BITS 64
DEFAULT REL
SECTION .text
GLOBAL bloxos_load_idt
bloxos_load_idt:
    lidt [rdi]
    ret
