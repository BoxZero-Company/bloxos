
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR is not set"),
    );

    let root = manifest_dir
        .parent()
        .expect("kernel directory must be inside repository")
        .to_path_buf();

    let out = PathBuf::from(
        env::var("OUT_DIR")
            .expect("OUT_DIR is not set"),
    );

    // NASM configuration
    let nasm = env::var_os("NASM")
        .unwrap_or_else(|| "nasm".into());

    println!("cargo:rerun-if-env-changed=NASM");

    let sources = [
        "boot/bootloader/entry.asm",
        "boot/bootloader/gdt.asm",
        "boot/memory/paging.asm",
        "boot/bootloader/idt.asm",
        "boot/bootloader/interrupts.asm",
        "boot/bootloader/cpu.asm",
        "boot/bootloader/multiboot.asm",
    ];

    for source in sources {
        let input = root.join(source);

        println!("cargo:rerun-if-changed={}", input.display());

        if !input.exists() {
            panic!("Assembly source not found: {}", input.display());
        }

        let object_name = source.replace('/', "_");
        let object = out.join(format!("{object_name}.o"));

        let status = Command::new(&nasm)
            .arg("-f")
            .arg("elf64")
            .arg(&input)
            .arg("-o")
            .arg(&object)
            .status()
            .unwrap_or_else(|error| {
                panic!("Could not start NASM: {error}");
            });

        if !status.success() {
            panic!("NASM failed: {}", input.display());
        }

        println!("cargo:rustc-link-arg={}", object.display());
    }

    // C application compilation using Clang
    let app_c = manifest_dir.join("apps/app.c");
    let app_h = manifest_dir.join("apps/app.h");

    println!("cargo:rerun-if-changed={}", app_c.display());
    println!("cargo:rerun-if-changed={}", app_h.display());

    if !app_c.exists() || !app_h.exists() {
        panic!("BloxOS C application source or header is missing");
    }

    let clang = env::var_os("CLANG")
        .unwrap_or_else(|| {
            "C:/Program Files/LLVM/bin/clang.exe".into()
        });

    println!("cargo:rerun-if-env-changed=CLANG");

    let app_object = out.join("bloxos_apps.o");

    let status = Command::new(&clang)
        .arg("--target=x86_64-unknown-none-elf")
        .arg("-c")
        .arg(&app_c)
        .arg("-o")
        .arg(&app_object)
        .arg("-I")
        .arg(manifest_dir.join("apps"))
        .arg("-ffreestanding")
        .arg("-fno-builtin")
        .arg("-fno-stack-protector")
        .arg("-mno-red-zone")
        .arg("-mcmodel=kernel")
        .arg("-fno-pic")
        .arg("-fno-pie")
        .arg("-O2")
        .status()
        .unwrap_or_else(|error| {
            panic!("Could not start Clang: {error}");
        });

    if !status.success() {
        panic!("Failed to compile BloxOS C application");
    }

    println!("cargo:rustc-link-arg={}", app_object.display());

    println!(
        "cargo:warning=BloxOS C application compiled for x86_64-unknown-none-elf"
    );

    // Linker script
    let linker_candidates = [
        root.join("boot/linker.ld"),
        root.join("system/linker.ld"),
    ];

    let linker_script = linker_candidates
        .iter()
        .find(|path| path.exists())
        .unwrap_or_else(|| {
            panic!(
                "linker.ld not found. Expected boot/linker.ld or system/linker.ld"
            )
        });

    println!("cargo:rerun-if-changed={}", linker_script.display());
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());

    println!(
        "cargo:warning=Using linker script: {}",
        linker_script.display()
    );
}
