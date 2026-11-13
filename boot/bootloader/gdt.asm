; Flat GDT: null, 64-bit code, data.
BITS 32
SECTION .rodata
ALIGN 8
GLOBAL gdt64_descriptor
 gdt64_start:
    dq 0x0000000000000000
    dq 0x00AF9A000000FFFF
    dq 0x00CF92000000FFFF
 gdt64_end:
 gdt64_descriptor:
    dw gdt64_end - gdt64_start - 1
    dd gdt64_start
