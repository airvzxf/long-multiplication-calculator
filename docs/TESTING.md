# Testing

## Layers

| Layer | Tooling | Where |
| --- | --- | --- |
| Unit (algorithm) | `cargo test` | `#[cfg(test)] mod tests` inside each `crates/core/src/*.rs` module |
| Doctests | `cargo test --doc` | `///` examples in every public function |
| CLI integration | `assert_cmd` + `predicates` + `tempfile` | `crates/cli/tests/` (added when needed) |
| WASM smoke | `wasm-pack test --headless --chrome` | `crates/wasm/src/lib.rs` (the `#[cfg(target_arch = "wasm32")]` test) |
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

# WASM bundle.
wasm-pack build crates/wasm --target web --release --no-opt

# Security audit.
cargo install cargo-audit --locked
cargo audit
```

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
/// use long_multiplication_core::store;
/// store("3", "2", "/tmp/x.txt").unwrap();
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
diff /tmp/new.txt tests/fixtures/13597_8642.txt
```

Fixtures live in `tests/fixtures/`. New fixtures should be added when
covering new sizes/edge cases (e.g. very small, very large, single
digit, carries that propagate through more than one column).
