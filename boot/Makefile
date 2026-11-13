
CARGO ?= cargo
RUST_TARGET ?= x86_64-unknown-none

GRUB_DIR ?= C:/grub
MSYS_DIR ?= C:/msys64

GRUB_MKSTANDALONE := $(GRUB_DIR)/grub-mkstandalone.exe
GRUB_HYBRID := $(GRUB_DIR)/i386-pc/boot_hybrid.img
GRUB_CDBOOT := $(GRUB_DIR)/i386-pc/cdboot.img
GRUB_BIOS_MODULES := $(GRUB_DIR)/i386-pc
GRUB_EFI_MODULES := $(GRUB_DIR)/x86_64-efi

XORRISO := $(MSYS_DIR)/usr/bin/xorriso.exe
DD := $(MSYS_DIR)/usr/bin/dd.exe
MFORMAT := $(MSYS_DIR)/ucrt64/bin/mformat.exe
MCOPY := $(MSYS_DIR)/ucrt64/bin/mcopy.exe
MMD := $(MSYS_DIR)/ucrt64/bin/mmd.exe

BUILD_DIR := build
ISO_ROOT := $(BUILD_DIR)/iso
GRUB_BUILD := $(BUILD_DIR)/grub

KERNEL := $(BUILD_DIR)/kernel.elf
EFI_IMAGE := $(ISO_ROOT)/EFI/efiboot.img
EFI_BOOT := $(GRUB_BUILD)/BOOTX64.EFI
BIOS_CORE := $(GRUB_BUILD)/core.img
BIOS_IMAGE := $(ISO_ROOT)/boot/grub/bios.img

.PHONY: all kernel iso clean check-tools check-iso-tools

all: kernel

check-tools:
	@command -v $(CARGO) >/dev/null || (echo "Rust/Cargo is required"; exit 1)

check-iso-tools:
	@test -f "$(GRUB_MKSTANDALONE)" || (echo "GRUB standalone missing"; exit 1)
	@test -f "$(GRUB_HYBRID)" || (echo "GRUB hybrid image missing"; exit 1)
	@test -f "$(GRUB_CDBOOT)" || (echo "GRUB CD boot image missing"; exit 1)
	@test -f "$(XORRISO)" || (echo "xorriso missing"; exit 1)
	@test -f "$(DD)" || (echo "dd missing"; exit 1)
	@test -f "$(MFORMAT)" || (echo "mformat missing"; exit 1)
	@test -f "$(MCOPY)" || (echo "mcopy missing"; exit 1)
	@test -f "$(MMD)" || (echo "mmd missing"; exit 1)

kernel: check-tools
	$(CARGO) build --manifest-path kernel/Cargo.toml --release --target $(RUST_TARGET)
	mkdir -p $(BUILD_DIR)
	cp kernel/target/$(RUST_TARGET)/release/bloxos-kernel $(KERNEL)

iso: kernel check-iso-tools
	rm -rf $(ISO_ROOT) $(GRUB_BUILD)
	mkdir -p $(ISO_ROOT)/boot/grub
	mkdir -p $(ISO_ROOT)/EFI
	mkdir -p $(GRUB_BUILD)

	cp $(KERNEL) $(ISO_ROOT)/boot/kernel.elf
	cp boot/grub/grub.cfg $(ISO_ROOT)/boot/grub/grub.cfg

	$(GRUB_MKSTANDALONE) -O i386-pc -d "$(GRUB_BIOS_MODULES)" --modules="normal configfile multiboot2 search search_fs_file iso9660 biosdisk part_msdos part_gpt" -o "$(BIOS_CORE)" "boot/grub/grub.cfg=boot/grub/grub.cfg"

	cat "$(GRUB_CDBOOT)" "$(BIOS_CORE)" > "$(BIOS_IMAGE)"

	$(GRUB_MKSTANDALONE) -O x86_64-efi -d "$(GRUB_EFI_MODULES)" --modules="normal configfile multiboot2 search search_fs_file iso9660 fat part_gpt" -o "$(EFI_BOOT)" "boot/grub/grub.cfg=boot/grub/grub.cfg"

	$(DD) if=/dev/zero of="$(EFI_IMAGE)" bs=1M count=16
	$(MFORMAT) -i "$(EFI_IMAGE)" -F ::
	$(MMD) -i "$(EFI_IMAGE)" ::/EFI
	$(MMD) -i "$(EFI_IMAGE)" ::/EFI/BOOT
	$(MCOPY) -i "$(EFI_IMAGE)" "$(EFI_BOOT)" ::/EFI/BOOT/BOOTX64.EFI

	$(XORRISO) -as mkisofs -R -J -joliet-long -V BLOXOS -o "$(BUILD_DIR)/bloxos.iso" -b boot/grub/bios.img -c boot/grub/boot.cat -no-emul-boot -boot-load-size 4 -boot-info-table --grub2-boot-info --grub2-mbr "$(GRUB_HYBRID)" -eltorito-alt-boot -e EFI/efiboot.img -no-emul-boot -append_partition 2 0xef "$(EFI_IMAGE)" -graft-points . /boot/grub/bios.img="$(BIOS_IMAGE)" /EFI/efiboot.img="$(EFI_IMAGE)"

	@echo.
	@echo ========================================
	@echo BloxOS ISO created successfully!
	@echo Output: build/bloxos.iso
	@echo ========================================

clean:
	rm -rf $(BUILD_DIR) kernel/target
