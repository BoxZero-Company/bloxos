; Multiboot2 constants and tiny boot-information helpers.
BITS 64
SECTION .text
GLOBAL bloxos_multiboot2_magic_valid
bloxos_multiboot2_magic_valid:
    xor eax, eax
    cmp edi, 0x36D76289
    sete al
    ret
