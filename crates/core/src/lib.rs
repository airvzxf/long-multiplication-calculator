//! Long Multiplication Calculator - pure algorithm core.
//!
//! This crate contains the algorithm and the Unicode table generator
//! for the long multiplication step-by-step layout (the one taught in
//! primary school in Mexico). It has no I/O, no terminal, no network,
//! and no dependencies on `clap`, `serde` or `wasm-bindgen`; it is
//! reusable from the CLI binary, from WebAssembly, and from any other
//! Rust project that links against it.
//!
//! # Quick start
//!
//! ```
//! use long_multiplication_core::{get_table, validate_input};
//!
//! let multiplicand = validate_input("13597", 1000).expect("valid");
//! let multiplier   = validate_input("8642",  1000).expect("valid");
//! let table = get_table(multiplicand, multiplier);
//! // 13597 * 8642 = 117505274; the product row reads
//! // "┃ 1 │ 1 │ 7 │ 5 │ 0 │ 5 │ 2 │ 7 │ 4 ┃ P".
//! assert!(table.contains("1 │ 1 │ 7 │ 5 │ 0 │ 5 │ 2 │ 7 │ 4 ┃ P"));
//! ```
//!
//! # The process as data
//!
//! [`LongMultiplication`] holds every step of the table (the units and
//! carries of each row, the column sums, each `Sub n.` section and the
//! product) and writes the table onto any [`core::fmt::Write`], so a
//! caller can stream it instead of building one big `String`.
//!
//! ```
//! use long_multiplication_core::LongMultiplication;
//!
//! let process = LongMultiplication::new("13597", "8642")?;
//! let product: String = process.product().iter().rev().map(|d| char::from(b'0' + d)).collect();
//! assert_eq!(product, "117505274");
//! assert_eq!(process.subtotals().len(), 1);
//! # Ok::<(), long_multiplication_core::CoreError>(())
//! ```
//!
//! # Errors
//!
//! Input validation is centralised in [`validate_input`]. It returns a
//! [`CoreError`] when the input is empty, contains a non-ASCII digit, or
//! exceeds the configured length. [`check_output_size`] returns
//! [`CoreError::OutputTooLarge`] when the rendered table would exceed a
//! byte budget.

#![forbid(unsafe_code)]
#![allow(missing_docs)]

pub mod breakdown;
mod error;
pub mod generate;
pub mod length;
mod multiplication;
mod process;

pub use error::{CoreError, Result};
pub use multiplication::{check_output_size, estimate_table_bytes, get_table};
pub use process::{LongMultiplication, PartialProduct, Subtotals};

/// Default maximum number of digits in a single operand.
///
/// This is a default, not a limit of the algorithm: pass a larger
/// value (or `usize::MAX`) to [`validate_input`] to accept longer
/// numbers. The number of digits alone does not bound the size of the
/// table (operands made of nines need one `Sub n.` section per digit),
/// so adapters should also check [`estimate_table_bytes`] or call
/// [`check_output_size`] before rendering.
pub const MAX_DIGITS: usize = 1_000;

/// Validate and normalize a numeric string for the algorithm.
///
/// Trims surrounding whitespace, rejects empty input, rejects any
/// non-ASCII-digit character (and reports its position), and enforces
/// the `max_digits` upper bound (pass `usize::MAX` for no limit). On success returns the trimmed
/// digit-only string as a `&str` borrowed from `input`, so no
/// allocation happens in the happy path.
///
/// # Errors
///
/// Returns [`CoreError::EmptyInput`], [`CoreError::TooLong`], or
/// [`CoreError::InvalidDigit`] describing the first failure found.
///
/// # Examples
///
/// ```
/// use long_multiplication_core::validate_input;
///
/// assert!(validate_input("  42  ", 1000).is_ok());
/// assert!(validate_input("",       1000).is_err());
/// assert!(validate_input("12a3",   1000).is_err());
/// ```
pub fn validate_input(input: &str, max_digits: usize) -> Result<&str> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(CoreError::EmptyInput);
    }
    for (pos, ch) in trimmed.chars().enumerate() {
        if !ch.is_ascii_digit() {
            return Err(CoreError::InvalidDigit { position: pos, character: ch });
        }
    }
    // Every character is an ASCII digit here, so bytes == digits.
    if trimmed.len() > max_digits {
        return Err(CoreError::TooLong { max: max_digits, got: trimmed.len() });
    }
    Ok(trimmed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_input_accepts_trimmed_digits() {
        assert_eq!(validate_input("  42  ", 1000).unwrap(), "42");
        assert_eq!(validate_input("0", 1000).unwrap(), "0");
        assert_eq!(validate_input("1234567890", 1000).unwrap(), "1234567890");
    }

    #[test]
    fn validate_input_rejects_empty() {
        assert!(matches!(validate_input("", 1000), Err(CoreError::EmptyInput)));
        assert!(matches!(validate_input("   ", 1000), Err(CoreError::EmptyInput)));
    }

    #[test]
    fn validate_input_rejects_non_digit() {
        match validate_input("12a3", 1000) {
            Err(CoreError::InvalidDigit { position, character }) => {
                assert_eq!(position, 2);
                assert_eq!(character, 'a');
            }
            other => panic!("expected InvalidDigit, got {other:?}"),
        }
        assert!(validate_input("1.5", 1000).is_err());
        assert!(validate_input("-3", 1000).is_err());
        assert!(validate_input(" 9 ", 1000).is_ok());
    }

    #[test]
    fn validate_input_rejects_too_long() {
        let big = "1".repeat(MAX_DIGITS + 1);
        match validate_input(&big, MAX_DIGITS) {
            Err(CoreError::TooLong { max, got }) => {
                assert_eq!(max, MAX_DIGITS);
                assert_eq!(got, MAX_DIGITS + 1);
            }
            other => panic!("expected TooLong, got {other:?}"),
        }
    }

    #[test]
    fn validate_input_reports_character_index_and_digit_count() {
        // A multi-byte character before the bad one must not shift the index.
        assert_eq!(
            validate_input("1é", 1000),
            Err(CoreError::InvalidDigit { position: 1, character: 'é' })
        );
        // An invalid character is reported even when the input is also too long.
        assert!(matches!(validate_input("ééé", 2), Err(CoreError::InvalidDigit { .. })));
    }

    #[test]
    fn validate_input_accepts_unlimited_length() {
        let big = "7".repeat(MAX_DIGITS * 3);
        assert_eq!(validate_input(&big, usize::MAX).map(str::len), Ok(MAX_DIGITS * 3));
    }

    #[test]
    fn get_table_contains_product() {
        let table = get_table("13597", "8642");
        // The product `117505274` appears in the final row, split by the
        // box-drawing separator. We just check the digits are all there.
        for digit in ['1', '7', '5', '0', '2', '7', '4'] {
            assert!(table.contains(digit), "table missing digit {digit}: {table}");
        }
    }
}
