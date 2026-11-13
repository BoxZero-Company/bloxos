#ifndef BLOXOS_BOOT_INFO_H
#define BLOXOS_BOOT_INFO_H
#include <stdint.h>
#define BLOXOS_MULTIBOOT2_BOOT_MAGIC UINT32_C(0x36D76289)
#define BLOXOS_MULTIBOOT2_TAG_END 0u
#define BLOXOS_MULTIBOOT2_TAG_MMAP 6u
#define BLOXOS_MULTIBOOT2_TAG_FRAMEBUFFER 8u
struct bloxos_multiboot2_info { uint32_t total_size; uint32_t reserved; };
struct bloxos_multiboot2_tag { uint32_t type; uint32_t size; };
struct bloxos_multiboot2_mmap_entry {
    uint64_t address; uint64_t length; uint32_t type; uint32_t reserved;
};
#endif
