//! WebAssembly bindings for the long-multiplication calculator.
//!
//! Exposes the same primitives the CLI uses: [`calculate`] takes two
//! digit strings and returns the Unicode table, [`max_digits`]
//! reports the upper bound, and [`version`] returns the
//! `CARGO_PKG_VERSION` baked into the binary.

use long_multiplication_core::{MAX_DIGITS, get_table, validate_input};
use wasm_bindgen::prelude::*;

/// Install a panic hook so that Rust panics surface readable
/// messages in the browser dev tools instead of just aborting.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Calculate the long-multiplication table for the two operands.
///
/// On success, returns the full Unicode table as a `String` (the
/// same byte-for-byte output as the CLI binary). On invalid input
/// or input that exceeds [`MAX_DIGITS`], returns a `JsError` whose
/// message describes the problem.
///
/// # Errors
///
/// Any [`long_multiplication_core::CoreError`] is converted to a
/// `JsError` so JavaScript can `try/catch` it.
#[wasm_bindgen]
pub fn calculate(multiplicand: &str, multiplier: &str) -> Result<String, JsError> {
    let a = validate_input(multiplicand, MAX_DIGITS).map_err(JsError::from)?;
    let b = validate_input(multiplier, MAX_DIGITS).map_err(JsError::from)?;
    Ok(get_table(a, b))
}

/// Maximum number of digits accepted by [`calculate`].
///
/// Exposed to JavaScript so the front end can show a friendlier
/// error before calling into WASM.
#[wasm_bindgen(js_name = maxDigits)]
pub fn max_digits() -> usize {
    MAX_DIGITS
}

/// Semantic version of the compiled WASM module.
///
/// Sourced from `CARGO_PKG_VERSION` at build time, so it always
/// matches the version in `Cargo.toml`. Exposed to JavaScript so
/// the front end can show it in the footer and so a runtime check
/// can confirm the binary that is actually executing matches the
/// one the page expects.
///
/// # Examples
///
/// ```
/// use long_multiplication_wasm::version;
///
/// // The string is the same one baked into Cargo.toml.
/// assert_eq!(version(), env!("CARGO_PKG_VERSION"));
/// assert!(!version().is_empty());
/// ```
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tell `wasm-bindgen-test` to run `#[wasm_bindgen_test]`-marked
    // tests in a browser (Chromium / Firefox) rather than in node.
    // Without this, the tests are skipped when `wasm-pack test` is
    // invoked with `--chrome` or `--firefox`.
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    /// `calculate` is a `#[wasm_bindgen]` shim that delegates to
    /// `long_multiplication_core`. The real coverage lives in
    /// `long-multiplication-core`'s tests (90 of them), which run
    /// natively with plain `cargo test`. The two tests below only
    /// sanity-check the bindgen glue: that the constants and the
    /// shim function are exported and the basic happy-path returns
    /// a non-empty string when compiled for `wasm32`.
    #[test]
    fn max_digits_is_one_thousand() {
        assert_eq!(max_digits(), 1000);
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen_test::wasm_bindgen_test]
    fn calculate_smoke() {
        let table = calculate("3", "2").expect("calculate");
        assert!(table.contains("6 ┃ P"));
    }
}
