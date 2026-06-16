//! Long Multiplication Calculator - command-line binary.
//!
//! Thin wrapper over [`long_multiplication_core::get_table`]. Parses
//! arguments, validates them, and either prints the table to stdout,
//! stores it to a file, or does both.

use anyhow::{Context, Result};
use clap::Parser;
use long_multiplication_core::{MAX_DIGITS, get_table, store, validate_input};
use std::io::{self, Write};
use std::path::PathBuf;

/// Long multiplication, step by step, like in school.
#[derive(Parser, Debug)]
#[command(name = "long-multiplication", version, about, long_about = None)]
#[command(after_help = "EXAMPLES:\n  \
    long-multiplication 13597 8642\n  \
    long-multiplication 5 7 --output store --file resultado.txt\n  \
    long-multiplication 99 99 --output both")]
struct Args {
    /// Multiplicando (solo dígitos 0-9)
    multiplicand: String,

    /// Multiplicador (solo dígitos 0-9)
    multiplier: String,

    /// Modo de salida: display, store o both
    #[arg(short, long, default_value = "display")]
    output: String,

    /// Archivo de salida (usado con --output store o both)
    #[arg(short, long, default_value = "long-multiplication-output.txt")]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let multiplicand = validate_input(&args.multiplicand, MAX_DIGITS)
        .with_context(|| format!("Invalid multiplicand '{}'", args.multiplicand))?;
    let multiplier = validate_input(&args.multiplier, MAX_DIGITS)
        .with_context(|| format!("Invalid multiplier '{}'", args.multiplier))?;

    let table = get_table(multiplicand, multiplier);

    match args.output.as_str() {
        "display" => {
            println!("{table}");
        }
        "store" => {
            let path = args.file.display().to_string();
            store(multiplicand, multiplier, &path)
                .with_context(|| format!("Cannot write to '{path}'"))?;
            eprintln!("Saved to {}", args.file.display());
        }
        "both" => {
            println!("{table}");
            let path = args.file.display().to_string();
            store(multiplicand, multiplier, &path)
                .with_context(|| format!("Cannot write to '{path}'"))?;
            eprintln!("Saved to {}", args.file.display());
        }
        other => {
            anyhow::bail!(
                "Invalid --output value '{other}': expected one of: display, store, both"
            );
        }
    }

    let _ = io::stdout().flush();
    Ok(())
}
