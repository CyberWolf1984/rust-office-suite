//! # Rust-Office Launcher
//!
//! Unified entry point that dispatches to Writer, Calc, or a start-centre
//! based on command-line arguments or the binary's executable name.

use std::env;
use std::process::Command;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("start-centre");

    match mode {
        "--writer" | "writer" => launch("ro-writer"),
        "--calc" | "calc" => launch("ro-calc"),
        "start-centre" | _ => {
            println!("╔══════════════════════════════════════╗");
            println!("║       Rust-Office Start Centre       ║");
            println!("╠══════════════════════════════════════╣");
            println!("║  1. Writer  (word processor)         ║");
            println!("║  2. Calc    (spreadsheet)            ║");
            println!("╚══════════════════════════════════════╝");
            println!();
            println!("Usage: rust-office --writer");
            println!("       rust-office --calc");
        }
    }
}

fn launch(binary: &str) {
    log::info!("Launching {binary}");

    // Resolve the binary next to the launcher executable.
    let mut path = env::current_exe().expect("cannot resolve own path");
    path.pop(); // remove the launcher filename
    path.push(binary);

    #[cfg(target_os = "windows")]
    {
        path.set_extension("exe");
    }

    match Command::new(&path).spawn() {
        Ok(_) => log::info!("{binary} launched successfully"),
        Err(e) => eprintln!("Failed to launch {}: {e}", path.display()),
    }
}
