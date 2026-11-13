
; ============================================================
; BloxOS 27H1 Horizon
; Multiboot2 Boot Entry
; x86 -> x86_64 Long Mode
; Framebuffer Request: 1024x768x32
; ============================================================

BITS 32

GLOBAL _start
GLOBAL pml4_table
GLOBAL pdpt_table
GLOBAL pd_table0
GLOBAL pd_table1
GLOBAL pd_table2
GLOBAL pd_table3

EXTERN kernel_main

; ============================================================
; MULTIBOOT2 HEADER
; ============================================================

SECTION .multiboot_header
ALIGN 8

multiboot2_header_start:
    dd 0xE85250D6
    dd 0
    dd multiboot2_header_end - multiboot2_header_start
    dd -(0xE85250D6 + (multiboot2_header_end - multiboot2_header_start))

    ; Framebuffer request
    dw 5
    dw 1
    dd 20
    dd 1024
    dd 768
    dd 32

    ALIGN 8

    ; End tag
    dw 0
    dw 0
    dd 8

multiboot2_header_end:

; ============================================================
; 32-BIT ENTRY
; ============================================================

SECTION .text
ALIGN 16

_start:
    cli
    cld

    ; Preserve Multiboot2 registers
    mov [boot_magic], eax
    mov [boot_info], ebx

    ; DEBUG A
    mov dx, 0xE9
    mov al, 'A'
    out dx, al

    ; Validate Multiboot2 magic
    cmp eax, 0x36D76289
    jne boot_halt

    ; DEBUG B
    mov dx, 0xE9
    mov al, 'B'
    out dx, al

    ; Setup stack
    mov esp, boot_stack_top
    and esp, 0xFFFFFFF0

    ; Build page tables
    call setup_page_tables

    ; DEBUG C
    mov dx, 0xE9
    mov al, 'C'
    out dx, al

    ; Load GDT
    lgdt [gdt64_descriptor]

    ; Enable PAE
    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax

    ; Load PML4
    mov eax, pml4_table
    mov cr3, eax

    ; Enable Long Mode
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)
    wrmsr

    ; Enable paging and protected mode
    mov eax, cr0
    or eax, (1 << 31) | 1
    mov cr0, eax

    ; DEBUG D
    mov dx, 0xE9
    mov al, 'D'
    out dx, al

    ; Far jump to 64-bit code segment
    jmp 0x08:long_mode_entry

; ============================================================
; PAGE TABLE INITIALIZATION
; Maps first 4 GiB using 2 MiB pages
; ============================================================

setup_page_tables:
    ; Clear all tables
    mov edi, pml4_table
    xor eax, eax
    mov ecx, (6 * 4096) / 4
    rep stosd

    ; PML4[0] -> PDPT
    mov eax, pdpt_table
    or eax, 0x03
    mov [pml4_table], eax

    ; PDPT entries -> page directories
    mov eax, pd_table0
    or eax, 0x03
    mov [pdpt_table + 0], eax

    mov eax, pd_table1
    or eax, 0x03
    mov [pdpt_table + 8], eax

    mov eax, pd_table2
    or eax, 0x03
    mov [pdpt_table + 16], eax

    mov eax, pd_table3
    or eax, 0x03
    mov [pdpt_table + 24], eax

    ; Fill 4 page directories
    ; Each entry maps one 2 MiB page.
    mov edi, pd_table0
    xor ebx, ebx

    mov ecx, 2048

.map_pages:
    mov eax, ebx
    or eax, 0x83
    mov [edi], eax
    mov dword [edi + 4], 0

    add ebx, 0x200000
    add edi, 8

    loop .map_pages

    ret

; ============================================================
; BOOT FAILURE
; ============================================================

boot_halt:
    mov dx, 0xE9
    mov al, '!'
    out dx, al

    cli

.halt_loop:
    hlt
    jmp .halt_loop

; ============================================================
; 64-BIT LONG MODE
; ============================================================

BITS 64

long_mode_entry:
    ; DEBUG E
    mov dx, 0xE9
    mov al, 'E'
    out dx, al

    ; Setup data segments
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov ss, ax

    xor eax, eax
    mov fs, ax
    mov gs, ax

    ; Setup stack
    mov rsp, boot_stack_top
    and rsp, -16

    xor rbp, rbp

    ; Pass Multiboot2 arguments
    mov edi, dword [rel boot_magic]
    mov esi, dword [rel boot_info]

    ; DEBUG F
    mov dx, 0xE9
    mov al, 'F'
    out dx, al

    ; Call Rust kernel
    call kernel_main

kernel_returned:
    mov dx, 0xE9
    mov al, 'R'
    out dx, al

    cli

.return_loop:
    hlt
    jmp .return_loop

; ============================================================
; GDT
; ============================================================

ALIGN 8

gdt64:
    dq 0x0000000000000000
    dq 0x00AF9A000000FFFF
    dq 0x00CF92000000FFFF

gdt64_end:

gdt64_descriptor:
    dw gdt64_end - gdt64 - 1
    dd gdt64

; ============================================================
; BOOT DATA
; ============================================================

SECTION .bss

ALIGN 4096

pml4_table:
    resb 4096

pdpt_table:
    resb 4096

pd_table0:
    resb 4096

pd_table1:
    resb 4096

pd_table2:
    resb 4096

pd_table3:
    resb 4096

ALIGN 16

boot_magic:
    resd 1

boot_info:
    resd 1

ALIGN 16

boot_stack_bottom:
    resb 65536

boot_stack_top:
