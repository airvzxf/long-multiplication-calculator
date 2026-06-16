# Changelog

All notable changes to this project are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/) and the
project adheres to [Semantic Versioning](https://semver.org/).

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
- `scripts/build_wasm.sh` for one-command WASM builds.

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
