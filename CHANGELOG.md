# Changelog

All notable changes to this project are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/) and the
project adheres to [Semantic Versioning](https://semver.org/).

## [2.1.0] - 2026-06-16

### Breaking changes

- **License metadata normalised to AGPL-3.0** in `Cargo.toml`,
  `README.md`, `web/index.html`, and the algorithm-emitted footer
  (`generate::author` and all derived fixtures). The repo's
  `LICENSE` file was already AGPL-3.0, but every other reference
  pointed at GPL-3.0. The footer of the rendered table now reads
  `License: AGPL-3.0`; consumers diffing the byte-for-byte output
  must regenerate their golden fixtures. The string version is
  bumped to `2.1.0` to surface the metadata change.
- **`core` is now strictly I/O-free.** `display` and `store` were
  removed from `long-multiplication-core` (they used `println!`
  and `std::fs::write`, contradicting the crate-boundary rule in
  `AGENTS.md`). The CLI now performs the file write itself via a
  small private `write_table` helper, and the `display` branch just
  inlines `println!`. No more re-exports of I/O helpers from
  `core`.

### Added

- `.github/workflows/deploy.yml` now installs `binaryen` before
  running `scripts/build_wasm.bash`, so Cloudflare Pages deploys
  publish the optimised WASM bundle (the script silently skipped
  `wasm-opt` if the binary was missing).

### Fixed

- `wasm-opt` failures in CI: rustc now emits WASM that uses
  sign-extension ops (`i32.extend8_s`, etc.). The CI workflow and
  `scripts/build_wasm.bash` now pass `--enable-sign-ext` alongside
  `--enable-bulk-memory`, so validation succeeds and the bundle
  is shrunk to ~41 KB.
- The CLI's `--output` flag is now a `clap::ValueEnum` with
  `Display`, `Store`, and `Both` variants validated at parse time
  (previously this was a `String` matched manually in `main`,
  contradicting the v2.0.0 changelog claim). Invalid values now
  produce a clap error (`error: invalid value '...' for '--output
  <OUTPUT>'`) instead of a generic `anyhow::bail!`, and `--help`
  lists the possible values with descriptions.
- Three `aria-describedby` attributes in `web/index.html` pointed
  to non-existent element ids (`multiplier-error`,
  `multiplicand-error`, `copy-hint`); the front end uses a single
  `role="status"` region for error feedback rather than per-field
  error nodes. The dead references are removed so screen readers
  stop announcing a broken description relationship.
- `docs/TESTING.md` referenced the golden fixture as
  `13597_8642.txt` (the real filename is `13597_x_8642.txt`) and
  the doctest example invoked the removed `core::store` helper.
  Both are updated to match the current codebase.

## [2.0.0] - 2026-06-16

### Breaking changes

- **Project restructured into a Cargo workspace** with three crates
  (`long-multiplication-core`, `long-multiplication` CLI binary,
  `long-multiplication-wasm`). The old monolithic `src/long-multiplication-command-line/`
  directory is gone.
- **Public function signatures changed.** All algorithm functions now
  take `&str` instead of `&String` (idiomatic Rust). New helper
  `validate_input(&str, max_digits) -> Result<&str, CoreError>`
  centralises input validation.
- **CLI binary renamed.** `long-multiplication-command-line` →
  `long-multiplication`. The CLI's user-facing behaviour is
  preserved (same args, same flags, same outputs), only the binary
  name changes.
- **Web frontend no longer depends on a CGI backend.** The
  `web/asset/bash/init.bash` and the bundled ELF binary
  (`web/asset/bin/long-multiplication-command-line`) are removed.
  Calculations happen in the browser via WebAssembly.

### Added

- Cargo workspace with `core`, `cli`, `wasm` crates.
- `validate_input` and `CoreError` (thiserror) in `core`, used by
  both the CLI and WASM adapters.
- Rust 2024 edition, MSRV 1.85.0, `rust-toolchain.toml` pinned to
  `stable`.
- GitHub Actions: CI (fmt + clippy + test + build + audit + WASM
  build) and CD (build WASM + deploy to Cloudflare Pages).
- Web: Content-Security-Policy meta tag, `aria-live` regions,
  `modulepreload`, ES module frontend, copy-to-clipboard, accessible
  labels, keyboard support.
- Cloudflare Pages config: `web/_headers` (caching + security
  headers) and `web/_redirects` (SPA fallback).
- `AGENTS.md` with the validation protocol.
- `docs/ARCHITECTURE.md`, `docs/DEPLOY.md`, `docs/TESTING.md`.
- `SECURITY.md` for vulnerability disclosure.
- `scripts/build_wasm.bash` for one-command WASM builds.

### Changed

- `cargo fmt`, `cargo clippy -D warnings`, `cargo test` and
  `cargo audit` run in CI on every push and PR.
- The CLI uses `clap` derive API (was Builder API), `anyhow` for
  error context, and an `OutputMode` enum validated by `clap` itself
  (no more "invalid mode" string match in `main`).
- The CLI replaces the `panic!` on `std::fs::write` errors with a
  proper `Result<()>` propagation.
- The release profile enables LTO, `panic = "abort"`, and
  `strip = "symbols"`.
- The web CSS now uses CSS variables with a dark-mode preference
  (`prefers-color-scheme`).

### Removed

- `src/long-multiplication-command-line/` (the old monolithic crate).
- `web/asset/bash/init.bash` (CGI script).
- `web/asset/bin/long-multiplication-command-line` (the 4.6 MB ELF
  binary checked into the repo).
- `web/.htaccess` (Apache-only configuration).
- `web/.idea/`, `src/long-multiplication-command-line/.idea/`
  (IntelliJ project files).
- `.idea/`, `.ftpquota` (IDE / FTP artifacts).

### Notes for users of v1.0.0

The CLI arguments, flags, and the format of the output table are
unchanged. The only externally visible change is the binary name
(`long-multiplication` instead of `long-multiplication-command-line`)
and the web frontend running in the browser instead of calling a
remote CGI.

## [1.0.0] - 2024-XX-XX

First public release. Single Rust binary + a static website that
called the binary via a Bash CGI on Apache.
