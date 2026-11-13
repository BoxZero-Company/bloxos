#ifndef BLOXOS_BOOT_H
#define BLOXOS_BOOT_H
#include <stdint.h>
#ifdef __cplusplus
extern "C" {
#endif
void bloxos_load_idt(const void *descriptor);
void bloxos_disable_interrupts(void);
void bloxos_enable_interrupts(void);
void bloxos_halt(void);
int bloxos_cpu_has_cpuid(void);
int bloxos_multiboot2_magic_valid(uint32_t magic);
#ifdef __cplusplus
}
#endif
#endif
