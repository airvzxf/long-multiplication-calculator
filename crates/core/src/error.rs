//! Error types for the long-multiplication core.

use thiserror::Error;

/// Result alias used throughout the core crate.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Errors produced by input validation and other core operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    /// The input string is empty or contains only whitespace.
    #[error("Input cannot be empty")]
    EmptyInput,

    /// The input string is longer than the configured maximum.
    #[error("Input exceeds the maximum length of {max} digits (got {got})")]
    TooLong {
        /// The configured maximum number of digits.
        max: usize,
        /// The number of digits in the offending input.
        got: usize,
    },

    /// The input string contains a character that is not an ASCII digit.
    #[error("Invalid character '{character}' at position {position}: expected 0-9")]
    InvalidDigit {
        /// Zero-based character index of the offending character within
        /// the trimmed input.
        position: usize,
        /// The offending character.
        character: char,
    },

    /// The rendered table would be larger than the configured maximum.
    #[error(
        "The table would take about {} (limit {})",
        format_bytes(*estimated),
        format_bytes(*max)
    )]
    OutputTooLarge {
        /// Upper-bound estimate of the table size, in bytes.
        estimated: usize,
        /// The configured maximum, in bytes.
        max: usize,
    },
}

/// Format a byte count with a binary unit, for example `1.5 MiB`.
///
/// The same wording as [`CoreError::OutputTooLarge`], for adapters
/// that report a size limit of their own (for example the CLI's JSON
/// output). Integer arithmetic only: floating-point formatting would
/// add about 15 KB to the WebAssembly bundle.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::format_bytes;
///
/// assert_eq!(format_bytes(512), "512 bytes");
/// assert_eq!(format_bytes(1536), "1.5 KiB");
/// assert_eq!(format_bytes(32 * 1024 * 1024), "32.0 MiB");
/// ```
pub fn format_bytes(bytes: usize) -> String {
    const UNITS: [&str; 5] = ["bytes", "KiB", "MiB", "GiB", "TiB"];
    let mut unit = 0;
    let mut scale: u128 = 1;
    while unit < UNITS.len() - 1 && bytes as u128 >= scale * 1024 {
        scale *= 1024;
        unit += 1;
    }
    if unit == 0 {
        return format!("{bytes} bytes");
    }
    // Round to one decimal place.
    let tenths = (bytes as u128 * 10 + scale / 2) / scale;
    format!("{}.{} {}", tenths / 10, tenths % 10, UNITS[unit])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn human_bytes_picks_a_binary_unit() {
        assert_eq!(format_bytes(512), "512 bytes");
        assert_eq!(format_bytes(1536), "1.5 KiB");
        assert_eq!(format_bytes(32 * 1024 * 1024), "32.0 MiB");
    }

    #[test]
    fn output_too_large_message_names_both_sizes() {
        let error = CoreError::OutputTooLarge { estimated: 3 * 1024 * 1024, max: 1024 * 1024 };
        assert_eq!(error.to_string(), "The table would take about 3.0 MiB (limit 1.0 MiB)");
    }
}
