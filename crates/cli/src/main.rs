//! Long Multiplication Calculator - command-line binary.
//!
//! Thin wrapper over [`long_multiplication_core::get_table`]. Parses
//! arguments, validates them, and either prints the table to stdout,
//! stores it to a file, or does both.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use long_multiplication_core::{MAX_DIGITS, get_table, validate_input};
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

    /// Modo de salida
    #[arg(short, long, value_enum, default_value_t = OutputMode::Display)]
    output: OutputMode,

    /// Archivo de salida (usado con --output store o both)
    #[arg(short, long, default_value = "long-multiplication-output.txt")]
    file: PathBuf,
}

/// Where the rendered table should be written.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lower")]
enum OutputMode {
    /// Print the table to stdout.
    Display,
    /// Save the table to `--file`.
    Store,
    /// Print to stdout and save to `--file`.
    Both,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let multiplicand = validate_input(&args.multiplicand, MAX_DIGITS)
        .with_context(|| format!("Invalid multiplicand '{}'", args.multiplicand))?;
    let multiplier = validate_input(&args.multiplier, MAX_DIGITS)
        .with_context(|| format!("Invalid multiplier '{}'", args.multiplier))?;

    let table = get_table(multiplicand, multiplier);

    match args.output {
        OutputMode::Display => {
            println!("{table}");
        }
        OutputMode::Store => {
            write_table(&args.file, &table)?;
            eprintln!("Saved to {}", args.file.display());
        }
        OutputMode::Both => {
            println!("{table}");
            write_table(&args.file, &table)?;
            eprintln!("Saved to {}", args.file.display());
        }
    }

    let _ = io::stdout().flush();
    Ok(())
}

/// Persist the rendered table to disk, attributing the path for context
/// on failure (replaces the old `core::store` helper, which was removed
/// to keep `core` I/O-free).
fn write_table(path: &std::path::Path, table: &str) -> Result<()> {
    std::fs::write(path, table).with_context(|| format!("Cannot write to '{}'", path.display()))?;
    Ok(())
}
