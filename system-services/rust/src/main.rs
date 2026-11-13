use std::env;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_status() {
    println!("BloxOS Rust service prototype");
    println!("Version: {}", VERSION);
    println!("OS: {}", env::consts::OS);
    println!("Architecture: {}", env::consts::ARCH);
    println!("Status: host-side prototype");
}

fn print_help() {
    println!("bloxos-service {}", VERSION);
    println!();
    println!("Usage:");
    println!("  bloxos-service status");
    println!("  bloxos-service --version");
    println!("  bloxos-service help");
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("status") => print_status(),
        Some("--version") | Some("-v") => {
            println!("bloxos-service {}", VERSION);
        }
        Some("help") | Some("--help") | Some("-h") | None => print_help(),
        Some(command) => {
            eprintln!("Unknown command: {}", command);
            eprintln!("Run 'bloxos-service help' for usage.");
            std::process::exit(2);
        }
    }
}
