//! Long Multiplication Calculator - command-line binary.
//!
//! Thin wrapper over [`long_multiplication_core::get_table`]. Parses
//! arguments, validates them, and either prints the table to stdout,
//! stores it to a file, or does both.

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use long_multiplication_core::{MAX_DIGITS, check_output_size, get_table, validate_input};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Default `--file` when `--output store` or `--output both` is used.
const DEFAULT_FILE: &str = "long-multiplication-output.txt";

/// Default `--max-output-mib`: 1 GiB, which fits in memory on any
/// desktop computer.
const DEFAULT_MAX_OUTPUT_MIB: usize = 1024;

/// Long multiplication, step by step, like in school.
#[derive(Parser, Debug)]
#[command(name = "long-multiplication", version, about, long_about = None)]
#[command(after_help = "LIMITS:\n  \
    The table grows quickly: two numbers made of 200 nines produce about 550 MiB.\n  \
    By default the CLI accepts up to 1000 digits per number and 1024 MiB of output.\n  \
    Use --max-digits / --max-output-mib (0 = no limit) or --unlimited to change that.\n  \
    The same settings can come from the environment variables shown above.\n\n\
EXAMPLES:\n  \
    long-multiplication 13597 8642\n  \
    long-multiplication 5 7 --output store --file result.txt\n  \
    long-multiplication 99 99 --output both\n  \
    long-multiplication --unlimited 999999 999999 > big.txt")]
struct Args {
    /// Multiplicand (digits 0-9 only)
    multiplicand: String,

    /// Multiplier (digits 0-9 only)
    multiplier: String,

    /// Where to write the table
    #[arg(short, long, value_enum, default_value_t = OutputMode::Display)]
    output: OutputMode,

    /// Output file for `--output store` or `--output both`
    /// [default: long-multiplication-output.txt]
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// Maximum digits per number (0 = no limit)
    #[arg(long, env = "LONG_MULTIPLICATION_MAX_DIGITS", default_value_t = MAX_DIGITS)]
    max_digits: usize,

    /// Maximum size of the table, in MiB (0 = no limit)
    #[arg(long, env = "LONG_MULTIPLICATION_MAX_OUTPUT_MIB", default_value_t = DEFAULT_MAX_OUTPUT_MIB)]
    max_output_mib: usize,

    /// Remove both limits; only memory, disk and time are the limit
    #[arg(long, env = "LONG_MULTIPLICATION_UNLIMITED")]
    unlimited: bool,
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

    if args.output == OutputMode::Display && args.file.is_some() {
        bail!("--file only applies to `--output store` or `--output both`");
    }

    let max_digits = limit(args.unlimited, args.max_digits);
    let multiplicand = validate_input(&args.multiplicand, max_digits)
        .with_context(|| format!("Invalid multiplicand '{}'", shorten(&args.multiplicand)))?;
    let multiplier = validate_input(&args.multiplier, max_digits)
        .with_context(|| format!("Invalid multiplier '{}'", shorten(&args.multiplier)))?;

    let max_output_mib = limit(args.unlimited, args.max_output_mib);
    if max_output_mib != usize::MAX {
        check_output_size(multiplicand, multiplier, max_output_mib.saturating_mul(1024 * 1024))
            .map_err(|error| {
                anyhow!("{error}; use --max-output-mib <MiB> or --unlimited to render it anyway")
            })?;
    }

    let table = get_table(multiplicand, multiplier);
    let file = args.file.unwrap_or_else(|| PathBuf::from(DEFAULT_FILE));

    if matches!(args.output, OutputMode::Store | OutputMode::Both) {
        write_table(&file, &table)?;
        eprintln!("Saved to {}", file.display());
    }
    if matches!(args.output, OutputMode::Display | OutputMode::Both) {
        print_table(&table)?;
    }

    Ok(())
}

/// Turn a CLI limit into the value the core expects: `0` or
/// `--unlimited` both mean "no limit".
fn limit(unlimited: bool, value: usize) -> usize {
    if unlimited || value == 0 { usize::MAX } else { value }
}

/// Keep error messages readable when someone pastes a huge number.
fn shorten(input: &str) -> String {
    const MAX: usize = 40;
    if input.chars().count() <= MAX {
        input.to_owned()
    } else {
        let head: String = input.chars().take(MAX).collect();
        format!("{head}… ({} characters)", input.chars().count())
    }
}

/// Print the table to stdout. A closed pipe (for example
/// `long-multiplication 99 99 | head`) is a normal way to stop
/// reading, so it ends the program quietly instead of panicking.
fn print_table(table: &str) -> Result<()> {
    let mut stdout = io::stdout().lock();
    match writeln!(stdout, "{table}").and_then(|()| stdout.flush()) {
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        result => result.context("Cannot write to stdout"),
    }
}

/// Persist the rendered table to disk, attributing the path for context
/// on failure. File I/O lives here, not in `core`, which stays I/O-free.
fn write_table(path: &Path, table: &str) -> Result<()> {
    std::fs::write(path, table).with_context(|| format!("Cannot write to '{}'", path.display()))
}
