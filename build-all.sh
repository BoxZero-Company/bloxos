
#!/usr/bin/env bash

set -Eeuo pipefail

# ==========================================
# BloxOS 27H1 Horizon - Clean Build
# Linux / Ubuntu
# ==========================================

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
cd "$ROOT_DIR"

BUILD_DIR="build"
ISO_DIR="$BUILD_DIR/iso"
GRUB_DIR="$BUILD_DIR/grub"

ISO_FILE="$BUILD_DIR/bloxos-27H1-Horizon.iso"
KERNEL_FILE="$BUILD_DIR/kernel.elf"

GRUB_PLATFORM_DIR="${GRUB_PLATFORM_DIR:-/usr/lib/grub/x86_64-efi}"

QEMU_BIN="${QEMU_BIN:-qemu-system-x86_64}"

# ==========================================
# COLORS
# ==========================================

GREEN='\033[0;32m'
RED='\033[0;31m'
CYAN='\033[0;36m'
YELLOW='\033[1;33m'
RESET='\033[0m'

info() {
    echo -e "${CYAN}$1${RESET}"
}

success() {
    echo -e "${GREEN}$1${RESET}"
}

warning() {
    echo -e "${YELLOW}$1${RESET}"
}

error() {
    echo -e "${RED}$1${RESET}"
}

fail() {
    error ""
    error "=========================================="
    error "BUILD FAILED"
    error "=========================================="
    error "$1"
    exit 1
}

trap 'fail "Unexpected error on line $LINENO: $BASH_COMMAND"' ERR

echo
echo "=========================================="
echo "       BloxOS 27H1 Horizon"
echo "          CLEAN BUILD"
echo "=========================================="
echo

# ==========================================
# HELPER: CHECK COMMAND
# ==========================================

check_command() {
    command -v "$1" >/dev/null 2>&1 || {
        error "Missing command: $1"
        return 1
    }
}

# ==========================================
# HELPER: FIND UEFI FIRMWARE
# ==========================================

find_uefi_firmware() {

    local candidates=(
        "/usr/share/OVMF/OVMF_CODE.fd"
        "/usr/share/OVMF/OVMF_CODE_4M.fd"
        "/usr/share/edk2/x64/OVMF_CODE.fd"
        "/usr/share/edk2/x64/OVMF_CODE.4m.fd"
        "/usr/share/qemu/OVMF_CODE.fd"
    )

    for firmware in "${candidates[@]}"; do
        if [[ -f "$firmware" ]]; then
            echo "$firmware"
            return 0
        fi
    done

    return 1
}

# ==========================================
# [1/7] CHECK TOOLS
# ==========================================

info "[1/7] Checking required tools..."

REQUIRED_TOOLS=(
    cargo
    rustc
    nasm
    grub-mkstandalone
    grub-file
    grub-script-check
    mformat
    mcopy
    mmd
    mdir
    xorriso
    truncate
    "$QEMU_BIN"
)

for tool in "${REQUIRED_TOOLS[@]}"; do
    check_command "$tool" || fail "Required tool not found: $tool"
done

if [[ ! -d "$GRUB_PLATFORM_DIR" ]]; then
    fail "GRUB EFI modules directory not found: $GRUB_PLATFORM_DIR"
fi

OVMF_FIRMWARE="$(find_uefi_firmware)" || \
    fail "UEFI firmware not found. Install OVMF."

success "Tools OK."
echo "GRUB modules: $GRUB_PLATFORM_DIR"
echo "UEFI firmware: $OVMF_FIRMWARE"

# ==========================================
# [2/7] CLEAN
# ==========================================

echo
info "[2/7] Cleaning old build..."

if [[ -d "$BUILD_DIR" ]]; then
    rm -rf "$BUILD_DIR"
fi

cargo clean

mkdir -p \
    "$BUILD_DIR/grub" \
    "$ISO_DIR/boot/grub" \
    "$ISO_DIR/EFI/BOOT"

success "Build directory cleaned."

# ==========================================
# [3/7] BUILD KERNEL
# ==========================================

echo
info "[3/7] Building Rust kernel..."

cargo build -p bloxos-kernel --target x86_64-unknown-none

BUILT_KERNEL="target/x86_64-unknown-none/debug/bloxos-kernel"

if [[ ! -f "$BUILT_KERNEL" ]]; then
    fail "Kernel binary not found: $BUILT_KERNEL"
fi

cp "$BUILT_KERNEL" "$KERNEL_FILE"

grub-file --is-x86-multiboot2 "$KERNEL_FILE" || \
    fail "Kernel is not a valid Multiboot2 kernel."

cp "$KERNEL_FILE" "$ISO_DIR/boot/kernel.elf"

success "Kernel build and Multiboot2 validation OK."

# ==========================================
# [4/7] GRUB CONFIGURATION
# ==========================================

echo
info "[4/7] Creating GRUB configuration..."

cat > "$ISO_DIR/boot/grub/grub.cfg" <<'EOF'
set default=0
set timeout=10
set timeout_style=menu
set menu_color_normal=white/black
set menu_color_highlight=black/light-green

insmod part_gpt
insmod part_msdos
insmod fat
insmod iso9660
insmod search
insmod search_fs_file
insmod multiboot2

menuentry "Try BloxOS 27H1 Horizon" --id bloxos-try {

    terminal_output console
    clear

    echo "Starting BloxOS 27H1 Horizon..."
    echo "Searching for kernel..."

    search --no-floppy --file --set=root /boot/kernel.elf

    if [ -f /boot/kernel.elf ]; then

        echo "[OK] Kernel found."
        echo "Loading BloxOS..."

        multiboot2 /boot/kernel.elf
        boot

    else

        echo "[ERROR] Kernel not found."
        sleep 5

    fi
}

menuentry "Install BloxOS 27H1 Horizon" --id bloxos-install {

    terminal_output console
    clear

    echo "BLOXOS 27H1 HORIZON INSTALLER"
    echo
    echo "Installer environment is not implemented yet."
    echo "Please use Try BloxOS to start the operating system."
    echo

    sleep 5
}

menuentry "BloxOS Recovery" --id bloxos-recovery {

    terminal_output console
    clear

    echo "BLOXOS RECOVERY"
    echo
    echo "Recovery environment is not implemented yet."
    echo

    sleep 5
}

menuentry "Restart System" --id bloxos-restart {
    reboot
}

menuentry "Shutdown System" --id bloxos-shutdown {
    halt
}
EOF

grub-script-check "$ISO_DIR/boot/grub/grub.cfg" || \
    fail "GRUB configuration validation failed."

success "GRUB configuration validation OK."

# ==========================================
# [5/7] BUILD UEFI GRUB
# ==========================================

echo
info "[5/7] Building standalone UEFI GRUB..."

grub-mkstandalone \
    -O x86_64-efi \
    -d "$GRUB_PLATFORM_DIR" \
    --modules="normal echo sleep multiboot2 search search_fs_file iso9660 part_gpt part_msdos fat video efi_gop all_video video_fb videoinfo" \
    -o "$GRUB_DIR/BOOTX64.EFI" \
    "boot/grub/grub.cfg=$ISO_DIR/boot/grub/grub.cfg"

if [[ ! -f "$GRUB_DIR/BOOTX64.EFI" ]]; then
    fail "BOOTX64.EFI was not generated."
fi

grub-file --is-x86_64-efi "$GRUB_DIR/BOOTX64.EFI" || \
    fail "UEFI GRUB validation failed."

success "UEFI GRUB validation OK."

# ==========================================
# [6/7] CREATE FAT EFI IMAGE
# ==========================================

echo
info "[6/7] Creating UEFI FAT image..."

EFI_IMAGE="$ISO_DIR/EFI/efiboot.img"

truncate -s 64M "$EFI_IMAGE"

mformat -i "$EFI_IMAGE" -F -v BLOXOS ::

mmd -i "$EFI_IMAGE" ::/EFI
mmd -i "$EFI_IMAGE" ::/EFI/BOOT

mcopy \
    -i "$EFI_IMAGE" \
    "$GRUB_DIR/BOOTX64.EFI" \
    ::/EFI/BOOT/BOOTX64.EFI

mdir -i "$EFI_IMAGE" ::/EFI/BOOT

success "UEFI FAT image created successfully."

# ==========================================
# [7/7] BUILD ISO
# ==========================================

echo
info "[7/7] Creating bootable ISO..."

xorriso \
    -as mkisofs \
    -R \
    -J \
    -joliet-long \
    -V BLOXOS \
    -o "$ISO_FILE" \
    -eltorito-alt-boot \
    -e EFI/efiboot.img \
    -no-emul-boot \
    -append_partition 2 0xef "$EFI_IMAGE" \
    "$ISO_DIR"

if [[ ! -f "$ISO_FILE" ]]; then
    fail "ISO file was not generated."
fi

success "ISO created successfully."

# ==========================================
# VERIFY ISO BOOT STRUCTURE
# ==========================================

echo
info "Verifying ISO boot structure..."

xorriso -indev "$ISO_FILE" -report_el_torito plain

success "ISO boot structure verification OK."

# ==========================================
# BUILD INFORMATION
# ==========================================

echo
echo "=========================================="
success "BUILD COMPLETED SUCCESSFULLY"
echo "=========================================="
echo

echo "ISO: $ISO_FILE"
echo "Kernel: $KERNEL_FILE"
echo "Size: $(du -h "$ISO_FILE" | cut -f1)"
echo "SHA256:"

sha256sum "$ISO_FILE"

# ==========================================
# QEMU
# ==========================================

echo
info "Starting BloxOS in QEMU..."
echo

"$QEMU_BIN" \
    -machine q35 \
    -accel tcg,thread=single,tb-size=64 \
    -m 512M \
    -smp 1 \
    -drive if=pflash,format=raw,unit=0,readonly=on,file="$OVMF_FIRMWARE" \
    -cdrom "$ISO_FILE" \
    -boot order=d,menu=on \
    -no-reboot \
    -no-shutdown \
    -debugcon stdio \
    -global isa-debugcon.iobase=0xe9

echo
success "BloxOS QEMU session ended."
