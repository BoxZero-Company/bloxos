# BloxOS boot subsystem

The kernel boot protocol is Multiboot2. GRUB enters `boot/bootloader/entry.asm` in 32-bit protected mode. The entry code validates the Multiboot2 magic, creates a three-level page-table hierarchy with a 1 GiB identity map, enables PAE and long mode, then transfers control to the Rust `kernel_main` entry in `kernel/src/main.rs`.

- `bootloader/entry.asm`: sole `_start` implementation and 32-to-64-bit transition.
- `bootloader/gdt.asm`: flat GDT and descriptor.
- `bootloader/idt.asm`: IDT load helper; the kernel must still construct/populate its IDT.
- `bootloader/interrupts.asm`: basic interrupt and halt helpers.
- `bootloader/cpu.asm`: CPUID availability check.
- `bootloader/multiboot.asm`: Multiboot2 magic helper.
- `memory/paging.asm`: bootstrap page tables.
- `memory/memory_map.inc`: Multiboot2 memory tag constants.
- `linker.ld`: kernel image layout.
- `grub/grub.cfg`: GRUB menu.

The current bootstrap is intentionally an early kernel foundation: it does not yet initialize a complete IDT, physical memory allocator, scheduler, storage stack, or graphical desktop. The identity map covers the first 1 GiB only. Do not enable interrupts until the kernel has installed valid interrupt handlers.
