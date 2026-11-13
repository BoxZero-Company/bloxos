
; ============================================================
; BloxOS 27H1 Horizon
; x86_64 Paging Setup
; Identity Map: First 4 GiB
; Page Size: 2 MiB
; ============================================================

BITS 32

SECTION .text

GLOBAL boot_setup_paging

EXTERN pml4_table
EXTERN pdpt_table

EXTERN pd_table0
EXTERN pd_table1
EXTERN pd_table2
EXTERN pd_table3


; ------------------------------------------------------------
; Macro: Map one 1 GiB region
; ------------------------------------------------------------

%macro MAP_PD 2

    mov edi, %1
    xor ecx, ecx

%%map_loop:

    mov eax, ecx
    shl eax, 21
    add eax, %2

    ; Present | Writable | Page Size (2 MiB)
    or eax, 0x83

    mov [edi + ecx * 8], eax
    mov dword [edi + ecx * 8 + 4], 0

    inc ecx
    cmp ecx, 512
    jne %%map_loop

%endmacro


; ------------------------------------------------------------
; Paging initialization
; ------------------------------------------------------------

boot_setup_paging:

    push edi
    push ecx

    ; Clear all six page-table pages
    mov edi, pml4_table
    xor eax, eax
    mov ecx, (4096 * 6) / 4
    rep stosd


    ; --------------------------------------------------------
    ; PML4[0] -> PDPT
    ; --------------------------------------------------------

    mov eax, pdpt_table
    or eax, 0x03
    mov [pml4_table], eax


    ; --------------------------------------------------------
    ; PDPT entries
    ; --------------------------------------------------------

    mov eax, pd_table0
    or eax, 0x03
    mov [pdpt_table], eax

    mov eax, pd_table1
    or eax, 0x03
    mov [pdpt_table + 8], eax

    mov eax, pd_table2
    or eax, 0x03
    mov [pdpt_table + 16], eax

    mov eax, pd_table3
    or eax, 0x03
    mov [pdpt_table + 24], eax


    ; --------------------------------------------------------
    ; Identity map 4 GiB
    ; --------------------------------------------------------

    MAP_PD pd_table0, 0x00000000
    MAP_PD pd_table1, 0x40000000
    MAP_PD pd_table2, 0x80000000
    MAP_PD pd_table3, 0xC0000000


    pop ecx
    pop edi

    ret
