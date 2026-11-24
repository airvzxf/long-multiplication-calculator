# Testing

## Layers

| Layer | Tooling | Where |
| --- | --- | --- |
| Unit (algorithm) | `cargo test` | `#[cfg(test)] mod tests` inside each `crates/core/src/*.rs` module |
| Doctests | `cargo test --doc` | `///` examples in every public function |
| Invariants (property tests) | `cargo test` + `proptest` | `crates/core/src/multiplication.rs`: every row has the same width, the product is right, the size estimate is an upper bound (256 random pairs of up to 60 digits, 64 nine-heavy pairs that need several `Sub n.` sections, plus fixed edge cases); a failure is shrunk to the smallest operands |
| Benchmarks | `cargo bench -p long-multiplication-core` (`criterion`) | `crates/core/benches/get_table.rs`: `get_table` (in memory), `write_table` in 256 KiB blocks (the CLI's streaming path) and `estimate_table_bytes`; local only, not in CI |
| Other platforms | CI `T2 · test (macOS)`, `T2 · test (Windows)` | the whole `cargo test --workspace`, golden fixtures included |
| CLI integration | `assert_cmd` + `predicates` + `tempfile` + `serde_json` | `crates/cli/tests/cli.rs` (text and JSON golden fixtures, limits, closed pipe, stored file) |
| WASM smoke | `wasm-pack test --headless --chrome` | `crates/wasm/src/lib.rs` (the `#[cfg(target_arch = "wasm32")]` tests; `calculateJson` must equal `tests/fixtures/*.json`) |
| Web end to end | `python3 scripts/e2e_web.py` (standard library only, chromedriver over HTTP) | the real page, served locally from a copy of `web/` with the version stamped and `web/_headers` applied, in English and Spanish: the 13597 x 8642 table against its fixture, the summary line, the shared link and Clear, the digit, length (< 5 s) and size errors, and no console errors (CSP violations included). It also checks that "Download JSON" saves the CLI's golden JSON, and walks every step of 13597 x 8642 and of twelve nines times twelve nines (10 `Sub n.` sections): each step must highlight exactly the digits its description names. CI runs it in `T2 · wasm` |
| Security audit | `cargo audit` | CI step |

## Running everything locally

```bash
# Fast feedback loop (covers unit + doctests in < 5s on a laptop).
cargo test --workspace

# Linting + formatting + docs.
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps

# Release build.
cargo build --workspace --release

# WASM bundle (the same script CI and the deploy use).
./scripts/build_wasm.bash

# The page end to end, after the WASM build (chromedriver and Chrome
# or Chromium; add --chrome-binary /usr/bin/chromium if needed).
python3 scripts/e2e_web.py

# Security audit.
cargo install cargo-audit --locked
cargo audit

# Benchmarks (optional, not in CI). Compare two runs on one machine:
cargo bench -p long-multiplication-core --bench get_table -- --save-baseline before
cargo bench -p long-multiplication-core --bench get_table -- --baseline before
```

If `proptest` finds a failing case it writes it to
`crates/core/proptest-regressions/`; commit that file with the fix so
the case is replayed on every run.

The full protocol is in [`AGENTS.md`](../AGENTS.md) ("The Gauntlet").

## Adding tests

### New unit test

Add it to the `#[cfg(test)] mod tests` block of the relevant
module. Keep the Arrange-Act-Assert pattern and a descriptive name:

```rust
#[test]
fn test_break_down_subtotal_with_thirteen_rows() {
    // Arrange
    let value: Vec<usize> = vec![1, 2, 3];
    let expected: Vec<usize> = vec![1, 2, 3];

    // Action
    let result = break_down_subtotal(&value);

    // Assert
    assert_eq!(expected, result);
}
```

### New doctest

Add a `# Examples` section to the public function's rustdoc. The
example must compile and run as part of `cargo test --doc`. If the
example triggers I/O, mark the block `no_run`:

```rust
/// ```no_run
/// use long_multiplication_core::get_table;
/// std::fs::write("/tmp/x.txt", get_table("3", "2")).unwrap();
/// ```
```

### New CLI integration test

Add a file under `crates/cli/tests/`. Use `assert_cmd` to invoke the
built binary and `predicates` to assert on stdout/stderr/exit code.
Use `tempfile` for any files the test creates.

## Byte-for-byte regression

The CLI output is contractual with v1.0.0. A simple shell test that
guards against accidental changes:

```bash
./target/debug/long-multiplication 13597 8642 > /tmp/new.txt
diff /tmp/new.txt tests/fixtures/13597_x_8642.txt
```

The JSON output is contractual too (`format_version` 1): every
`tests/fixtures/*.json` must match `--format json` byte for byte.

Fixtures live in `tests/fixtures/`. New fixtures should be added when
covering new sizes/edge cases (e.g. very small, very large, single
digit, carries that propagate through more than one column).
`999999999999_x_999999999999.txt` covers ten `Sub n.` sections.
