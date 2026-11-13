
BITS 64
SECTION .text

GLOBAL bloxos_disable_interrupts
GLOBAL bloxos_enable_interrupts
GLOBAL bloxos_halt

; Disable CPU interrupts.
bloxos_disable_interrupts:
    cli
    ret

; Enable CPU interrupts.
bloxos_enable_interrupts:
    sti
    ret

; Halt the CPU permanently.
bloxos_halt:
    cli
.halt_loop:
    hlt
    jmp .halt_loop
