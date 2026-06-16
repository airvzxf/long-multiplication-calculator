# AGENTS.md

Operating instructions for AI coding agents (and humans) working on
this repository. Following these keeps the CI gate green and avoids
regressions.

## The Gauntlet (run before every commit)

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
cargo doc --workspace --no-deps
cargo audit
./scripts/build_wasm.sh        # build + wasm-opt -Oz into web/asset/js/wasm/
wasm-pack test --chrome --headless crates/wasm   # or --firefox --headless
```

The CI workflow (`.github/workflows/ci.yml`) runs the same checks on
every push. Locally you can skip `cargo audit` and the WASM build if
you are not touching dependencies or `crates/wasm`.

> **Note on the WASM test runner:** the `calculate_smoke` test is
> configured with `wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser)`,
> so it runs in headless Chromium by default. If you only have
> Firefox installed, `wasm-pack test --firefox --headless crates/wasm`
> works too. The test is skipped with a hint if you invoke
> `wasm-pack test` without `--chrome` / `--firefox` / `--headless`
> while the browser-config macro is in effect.

## Toolchain

The project pins `stable` Rust in `rust-toolchain.toml`. MSRV is
`1.85.0` (enforced via `.clippy.toml` and `package.rust-version`).
Required components: `clippy`, `rustfmt`. Required target for WASM:
`wasm32-unknown-unknown`.

## Crate boundaries (strict)

```
core  ← cli
core  ← wasm
core  must not depend on clap, serde, wasm-bindgen, std::fs, println!,
       or unwrap()/expect() in production code.
```

If you need to add a dependency to `core`, ask first. Prefer to add
it to `cli` or `wasm` instead. `core` is a pure library.

## Test discipline

- **Algorithm tests** live next to the code in `#[cfg(test)] mod tests`
  blocks within each module. Keep them there.
- **Doctests** are first-class: any new public function must have a
  `# Examples` section in its rustdoc. Run `cargo test --doc` to verify.
- **Byte-for-byte output is contractual.** The v1.0.0 CLI output is
  the golden reference. If you change the algorithm and the output
  changes, document why in `CHANGELOG.md` and consider a major version
  bump.
- **No `unwrap()` or `expect()` in production code** (i.e. anything
  that is not `#[cfg(test)]` or a doctest). Use `Result<T, CoreError>`
  in `core` and `anyhow::Result` in `cli`.

## Release checklist

1. `cargo +stable build --workspace --release` passes.
2. `wasm-pack build crates/wasm --target web --release` produces
   a bundle < 100 KB unoptimised, < 50 KB with `wasm-opt -Oz`.
3. All 50+ unit tests pass; all doctests pass.
4. `CHANGELOG.md` updated with a new `## [X.Y.Z]` entry.
5. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`.
6. Confirm Cloudflare Pages redeployed automatically (or trigger
   via `Actions → Deploy to Cloudflare Pages → Run workflow`).

## Common pitfalls

- **WASM target missing:** `rustup target add wasm32-unknown-unknown`.
- **`wasm-pack` not found:** install from
  <https://rustwasm.github.io/wasm-pack/>.
- **`wasm-opt` aborts with `memory.copy operations require bulk memory operations`:** the
  Rust toolchain emits WASM that uses the bulk-memory proposal. Pass
  `--enable-bulk-memory` to `wasm-opt` (the `scripts/build_wasm.sh`
  script does this for you). wasm-pack's default `-O` invocation
  does not enable bulk-memory, so do **not** let wasm-pack call
  wasm-opt on its own — use `--no-opt` and run `wasm-opt` yourself.
- **Clippy lints failing:** see the per-crate `[lints.clippy]`
  sections in `Cargo.toml`. The project allows a small set of
  `clippy::all` lints that match the original 2021 code style
  (e.g. `needless_late_init`, `single_char_add_str`).
- **`cargo audit` failing on an existing dependency:** discuss before
  upgrading; SemVer 2.0.0 will be bumped if a fix requires a major
  version of a transitive dep.
