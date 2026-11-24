//! Long Multiplication Calculator - command-line binary.
//!
//! Thin wrapper over [`long_multiplication_core::LongMultiplication`].
//! Parses arguments, validates them, and streams the table (or the
//! JSON process) to stdout, to a file, or to both, without holding
//! the whole output in memory.

mod json;
mod output;

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, ValueEnum};
use long_multiplication_core::{
    CoreError, LongMultiplication, MAX_DIGITS, format_bytes, validate_input,
};
use output::{Sink, SinkError};
use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

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
    long-multiplication --format json 123 456\n  \
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
    /// [default: long-multiplication-output.txt, or .json with --format json]
    #[arg(short, long)]
    file: Option<PathBuf>,

    /// What to write: the table, or every step of the process as JSON
    #[arg(long, value_enum, env = "LONG_MULTIPLICATION_FORMAT", default_value_t = Format::Text)]
    format: Format,

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

/// What the output contains.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "lower")]
enum Format {
    /// The Unicode table.
    Text,
    /// The whole process as JSON: rows, column sums, subtotals, product.
    Json,
}

impl Format {
    /// Default `--file` when `--output store` or `--output both` is used.
    fn default_file(self) -> &'static str {
        match self {
            Format::Text => "long-multiplication-output.txt",
            Format::Json => "long-multiplication-output.json",
        }
    }
}

/// Where the output should be written.
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

    let process = LongMultiplication::new(multiplicand, multiplier)?;

    let max_output_mib = limit(args.unlimited, args.max_output_mib);
    if max_output_mib != usize::MAX {
        let max = max_output_mib.saturating_mul(1024 * 1024);
        let estimated = match args.format {
            Format::Text => process.estimate_table_bytes(),
            Format::Json => json::estimate_json_bytes(&process),
        };
        if estimated > max {
            let error = match args.format {
                Format::Text => CoreError::OutputTooLarge { estimated, max }.to_string(),
                Format::Json => format!(
                    "The JSON output would take about {} (limit {})",
                    format_bytes(estimated),
                    format_bytes(max)
                ),
            };
            bail!("{error}; use --max-output-mib <MiB> or --unlimited to render it anyway");
        }
    }

    let store = matches!(args.output, OutputMode::Store | OutputMode::Both);
    let display = matches!(args.output, OutputMode::Display | OutputMode::Both);
    let path = args.file.unwrap_or_else(|| PathBuf::from(args.format.default_file()));
    let file = if store {
        Some(File::create(&path).map_err(|error| file_error(&path, error))?)
    } else {
        None
    };

    let mut sink = Sink::new(display, file);
    let written = match args.format {
        Format::Text => process.write_table(&mut sink),
        Format::Json => json::write_json(&process, &mut sink),
    };
    if written.is_ok() && args.format == Format::Text {
        // The text format has always printed one more newline than it
        // stores; both outputs stay byte for byte as in v1.0.0.
        sink.stdout_only("\n");
    }
    // A failed write is reported here; a closed stdout pipe (for
    // example `long-multiplication 99 99 | head`) ends quietly.
    match sink.finish() {
        Ok(()) => {}
        Err(SinkError::File(error)) => return Err(file_error(&path, error)),
        Err(SinkError::Stdout(error)) => return Err(error).context("Cannot write to stdout"),
    }

    if store {
        eprintln!("Saved to {}", path.display());
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

/// The error for a file that cannot be written, with a `mkdir -p`
/// hint when its directory does not exist. File I/O lives here, not
/// in `core`, which stays I/O-free.
fn file_error(path: &Path, error: io::Error) -> anyhow::Error {
    let context = format!("Cannot write to '{}'", path.display());
    match missing_parent(path, &error) {
        Some(parent) => anyhow!(error).context(format!(
            "{context}: the directory '{}' does not exist (create it with `{}`)",
            parent.display(),
            mkdir_command(parent)
        )),
        None => anyhow!(error).context(context),
    }
}

/// The command that creates `dir` and its missing parents, quoted for
/// the usual shell of the platform.
fn mkdir_command(dir: &Path) -> String {
    let dir = dir.display().to_string();
    if cfg!(windows) {
        // `mkdir` in cmd.exe and PowerShell creates the parents too.
        return format!("mkdir \"{dir}\"");
    }
    let plain = dir.chars().all(|c| c.is_ascii_alphanumeric() || "/._-+,:@%=".contains(c));
    if plain && !dir.is_empty() {
        format!("mkdir -p {dir}")
    } else {
        format!("mkdir -p '{}'", dir.replace('\'', r"'\''"))
    }
}

/// The parent directory of `path`, when a write failed because that
/// directory does not exist.
fn missing_parent<'a>(path: &'a Path, error: &io::Error) -> Option<&'a Path> {
    if error.kind() != io::ErrorKind::NotFound {
        return None;
    }
    path.parent().filter(|parent| !parent.as_os_str().is_empty() && !parent.exists())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn mkdir_command_quotes_only_when_needed() {
        assert_eq!(mkdir_command(Path::new("out/tables")), "mkdir -p out/tables");
        assert_eq!(mkdir_command(Path::new("my tables")), "mkdir -p 'my tables'");
        assert_eq!(mkdir_command(Path::new("it's")), r"mkdir -p 'it'\''s'");
    }

    #[cfg(windows)]
    #[test]
    fn mkdir_command_uses_the_windows_syntax() {
        assert_eq!(mkdir_command(Path::new(r"out\my tables")), r#"mkdir "out\my tables""#);
    }
}
