// ---
// tags: strata, rust, cli
// crystal-type: source
// crystal-domain: comp
// ---
//! strata — one door to the five algebras.
//!
//!   strata nebu   …   Goldilocks field 𝔽ₚ
//!   strata jali   …   R_q polynomial ring
//!   strata kuro   …   𝔽₂ tower field
//!   strata trop   …   tropical (min,+)
//!   strata genies …   𝔽_q isogeny field
//!
//! `strata <algebra> [args…]` forwards to the algebra's CLI — the binary built
//! alongside this one, so no lookup beyond our own directory.

use std::io::IsTerminal;
use std::process::{exit, Command};

const ALGEBRAS: &[(&str, &str)] = &[
    ("nebu", "Goldilocks field 𝔽ₚ arithmetic"),
    ("jali", "R_q polynomial ring"),
    ("kuro", "𝔽₂ tower field for binary proving"),
    ("trop", "tropical (min,+) algebra"),
    ("genies", "𝔽_q isogeny field"),
];

fn tty() -> bool {
    std::io::stdout().is_terminal()
}
fn paint(code: &str, s: &str) -> String {
    if tty() { format!("\x1b[{code}m{s}\x1b[0m") } else { s.to_string() }
}
fn dim(s: &str) -> String {
    paint("90", s)
}
fn bold(s: &str) -> String {
    paint("1", s)
}

const LOGO: &str = "\
\x1b[31m███████╗████████╗██████╗  █████╗ ████████╗ █████╗ \x1b[0m
\x1b[33m██╔════╝╚══██╔══╝██╔══██╗██╔══██╗╚══██╔══╝██╔══██╗\x1b[0m
\x1b[32m███████╗   ██║   ██████╔╝███████║   ██║   ███████║\x1b[0m
\x1b[36m╚════██║   ██║   ██╔══██╗██╔══██║   ██║   ██╔══██║\x1b[0m
\x1b[34m███████║   ██║   ██║  ██║██║  ██║   ██║   ██║  ██║\x1b[0m
\x1b[35m╚══════╝   ╚═╝   ╚═╝  ╚═╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝\x1b[0m";

fn help() {
    if tty() {
        println!("{LOGO}");
        println!("{}\n", paint("37", "    the five algebras"));
    }
    println!("{}", dim("usage: strata <algebra> [args…]"));
    for (name, desc) in ALGEBRAS {
        println!("  {}  {}", bold(&format!("{name:<7}")), dim(desc));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let sub = args.first().map(String::as_str).unwrap_or("");

    if matches!(sub, "" | "help" | "--help" | "-h") {
        help();
        return;
    }
    if !ALGEBRAS.iter().any(|(a, _)| *a == sub) {
        eprintln!("strata: unknown algebra '{sub}' — try `strata help`");
        exit(2);
    }

    // The algebra binary sits next to us (same target/release dir). Its name is
    // either the algebra (nebu/jali/kuro) or `<algebra>-cli` (trop/genies).
    // Canonicalize first: when invoked via a `~/.cargo/bin` symlink, resolve it
    // back to the real build directory where the siblings live.
    let dir = std::env::current_exe()
        .ok()
        .and_then(|e| std::fs::canonicalize(&e).ok().or(Some(e)))
        .and_then(|e| e.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default();
    let target = [dir.join(sub), dir.join(format!("{sub}-cli"))]
        .into_iter()
        .find(|p| p.exists());

    match target {
        Some(bin) => match Command::new(&bin).args(&args[1..]).status() {
            Ok(status) => exit(status.code().unwrap_or(0)),
            Err(e) => {
                eprintln!("strata: cannot run {sub}: {e}");
                exit(1);
            }
        },
        None => {
            eprintln!("strata: {sub} is not built — run `cy install strata`");
            exit(1);
        }
    }
}
