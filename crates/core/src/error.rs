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
        /// The actual length of the offending input.
        got: usize,
    },

    /// The input string contains a character that is not an ASCII digit.
    #[error("Invalid character '{character}' at position {position}: expected 0-9")]
    InvalidDigit {
        /// Byte offset of the offending character within the trimmed input.
        position: usize,
        /// The offending character.
        character: char,
    },
}
