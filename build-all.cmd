
@echo off
setlocal EnableExtensions
title BloxOS 27H1 Horizon - Clean Build

cd /d "%~dp0"

echo.
echo ==========================================
echo       BloxOS 27H1 Horizon
echo          CLEAN BUILD
echo ==========================================
echo.

REM ==========================================
REM [1/7] CHECK TOOLS
REM ==========================================

echo [1/7] Checking required tools...

where cargo >nul 2>&1 || goto :tool_error
where rustc >nul 2>&1 || goto :tool_error
where nasm >nul 2>&1 || goto :tool_error

if not exist "C:\grub\grub-mkstandalone.exe" goto :tool_error
if not exist "C:\grub\grub-file.exe" goto :tool_error
if not exist "C:\grub\grub-script-check.exe" goto :tool_error
if not exist "C:\msys64\ucrt64\bin\mformat.exe" goto :tool_error
if not exist "C:\msys64\ucrt64\bin\mcopy.exe" goto :tool_error
if not exist "C:\msys64\ucrt64\bin\mmd.exe" goto :tool_error
if not exist "C:\msys64\ucrt64\bin\mdir.exe" goto :tool_error
if not exist "C:\msys64\usr\bin\xorriso.exe" goto :tool_error
if not exist "C:\Program Files\qemu\qemu-system-x86_64.exe" goto :tool_error
if not exist "C:\Program Files\qemu\share\edk2-x86_64-code.fd" goto :tool_error

echo Tools OK.

REM ==========================================
REM [2/7] CLEAN
REM ==========================================

echo.
echo [2/7] Cleaning old build...

if exist build rmdir /s /q build

if errorlevel 1 goto :error

cargo clean

if errorlevel 1 goto :error

mkdir build
mkdir build\grub
mkdir build\iso
mkdir build\iso\boot
mkdir build\iso\boot\grub
mkdir build\iso\EFI
mkdir build\iso\EFI\BOOT

if errorlevel 1 goto :error

REM ==========================================
REM [3/7] BUILD KERNEL
REM ==========================================

echo.
echo [3/7] Building Rust kernel...

cargo build -p bloxos-kernel --target x86_64-unknown-none

if errorlevel 1 goto :error

if not exist "target\x86_64-unknown-none\debug\bloxos-kernel" goto :error

copy /Y "target\x86_64-unknown-none\debug\bloxos-kernel" "build\kernel.elf" >nul

if errorlevel 1 goto :error

C:\grub\grub-file.exe --is-x86-multiboot2 build\kernel.elf

if errorlevel 1 goto :error

copy /Y build\kernel.elf build\iso\boot\kernel.elf >nul

if errorlevel 1 goto :error

echo Kernel build and Multiboot2 validation OK.



REM ==========================================
REM [4/7] GRUB CONFIGURATION
REM ==========================================

echo.
echo [4/7] Creating GRUB configuration...

(
echo set default=0
echo set timeout=10
echo set timeout_style=menu
echo set menu_color_normal=white/black
echo set menu_color_highlight=black/light-green
echo.
echo insmod part_gpt
echo insmod part_msdos
echo insmod fat
echo insmod iso9660
echo insmod search
echo insmod search_fs_file
echo insmod multiboot2
echo.
echo menuentry "Try BloxOS 27H1 Horizon" --id bloxos-try {
echo     terminal_output console
echo     clear
echo     echo "Starting BloxOS 27H1 Horizon..."
echo     echo "Searching for kernel..."
echo     search --no-floppy --file --set=root /boot/kernel.elf
echo     if [ -f /boot/kernel.elf ]; then
echo         echo "[OK] Kernel found."
echo         echo "Loading BloxOS..."
echo         multiboot2 /boot/kernel.elf
echo         boot
echo     else
echo         echo "[ERROR] Kernel not found."
echo         sleep 5
echo     fi
echo }
echo.
echo menuentry "Install BloxOS 27H1 Horizon" --id bloxos-install {
echo     terminal_output console
echo     clear
echo     echo "BLOXOS 27H1 HORIZON INSTALLER"
echo     echo.
echo     echo "Installer environment is not implemented yet."
echo     echo "Please use Try BloxOS to start the operating system."
echo     echo.
echo     sleep 5
echo }
echo.
echo menuentry "BloxOS Recovery" --id bloxos-recovery {
echo     terminal_output console
echo     clear
echo     echo "BLOXOS RECOVERY"
echo     echo.
echo     echo "Recovery environment is not implemented yet."
echo     echo.
echo     sleep 5
echo }
echo.
echo menuentry "Restart System" --id bloxos-restart {
echo     reboot
echo }
echo.
echo menuentry "Shutdown System" --id bloxos-shutdown {
echo     halt
echo }
) > build\iso\boot\grub\grub.cfg

if errorlevel 1 goto :error

C:\grub\grub-script-check.exe build\iso\boot\grub\grub.cfg

if errorlevel 1 goto :error

echo GRUB configuration validation OK.

REM ==========================================
REM [5/7] BUILD UEFI GRUB
REM ==========================================

echo.
echo [5/7] Building standalone UEFI GRUB...

C:\grub\grub-mkstandalone.exe -O x86_64-efi -d C:\grub\x86_64-efi --modules="normal echo sleep multiboot2 search search_fs_file iso9660 part_gpt part_msdos fat video efi_gop all_video video_fb videoinfo" -o build\grub\BOOTX64.EFI boot/grub/grub.cfg=build/iso/boot/grub/grub.cfg

if errorlevel 1 goto :error

if not exist build\grub\BOOTX64.EFI goto :error

C:\grub\grub-file.exe --is-x86_64-efi build\grub\BOOTX64.EFI

if errorlevel 1 goto :error

echo UEFI GRUB validation OK.

REM ==========================================
REM [6/7] CREATE FAT EFI IMAGE
REM ==========================================

echo.
echo [6/7] Creating UEFI FAT image...

fsutil file createnew build\iso\EFI\efiboot.img 67108864

if errorlevel 1 goto :error

C:\msys64\ucrt64\bin\mformat.exe -i build\iso\EFI\efiboot.img -F -v BLOXOS ::

if errorlevel 1 goto :error

C:\msys64\ucrt64\bin\mmd.exe -i build\iso\EFI\efiboot.img ::/EFI

if errorlevel 1 goto :error

C:\msys64\ucrt64\bin\mmd.exe -i build\iso\EFI\efiboot.img ::/EFI/BOOT

if errorlevel 1 goto :error

C:\msys64\ucrt64\bin\mcopy.exe -i build\iso\EFI\efiboot.img build\grub\BOOTX64.EFI ::/EFI/BOOT/BOOTX64.EFI

if errorlevel 1 goto :error

C:\msys64\ucrt64\bin\mdir.exe -i build\iso\EFI\efiboot.img ::/EFI/BOOT

if errorlevel 1 goto :error

REM ==========================================
REM [7/7] BUILD ISO
REM ==========================================

echo.
echo [7/7] Creating bootable ISO...

"C:\msys64\usr\bin\xorriso.exe" -as mkisofs -R -J -joliet-long -V BLOXOS -o build\bloxos-27H1-Horizon.iso -eltorito-alt-boot -e EFI/efiboot.img -no-emul-boot -append_partition 2 0xef build\iso\EFI\efiboot.img build\iso

if errorlevel 1 goto :error

if not exist build\bloxos-27H1-Horizon.iso goto :error

echo.
echo ==========================================
echo BUILD COMPLETED SUCCESSFULLY
echo ==========================================
echo.
echo ISO: build\bloxos-27H1-Horizon.iso
echo.

REM ==========================================
REM VERIFY ISO BOOT STRUCTURE
REM ==========================================

"C:\msys64\usr\bin\xorriso.exe" -indev build\bloxos-27H1-Horizon.iso -report_el_torito plain

if errorlevel 1 goto :error

echo.
echo Starting BloxOS in QEMU...
echo.

REM ==========================================
REM QEMU
REM ==========================================

"C:\Program Files\qemu\qemu-system-x86_64.exe" -machine q35 -accel tcg,thread=single,tb-size=64 -m 512M -smp 1 -drive if=pflash,format=raw,unit=0,readonly=on,file="C:\Program Files\qemu\share\edk2-x86_64-code.fd" -cdrom build\bloxos-27H1-Horizon.iso -boot order=d,menu=on -no-reboot -no-shutdown -debugcon stdio -global isa-debugcon.iobase=0xe9

goto :end

REM ==========================================
REM ERROR HANDLING
REM ==========================================

:tool_error
echo.
echo ERROR: A required tool is missing.
echo Check Rust, NASM, GRUB, MSYS2 and QEMU.
pause
goto :end

:error
echo.
echo ==========================================
echo BUILD FAILED
echo ==========================================
echo Check the command immediately above.
pause
goto :end

:end
endlocal
