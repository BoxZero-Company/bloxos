BITS 64
SECTION .text
GLOBAL bloxos_cpu_has_cpuid
; Returns 1 in EAX when CPUID is available, otherwise 0.
bloxos_cpu_has_cpuid:
    pushfq
    pop rax
    mov rcx, rax
    xor rax, 1 << 21
    push rax
    popfq
    pushfq
    pop rax
    xor rax, rcx
    shr rax, 21
    and eax, 1
    push rcx
    popfq
    ret
