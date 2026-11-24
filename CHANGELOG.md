# Changelog

All notable changes to this project are documented here. The format
follows [Keep a Changelog](https://keepachangelog.com/) and the
project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

The text table and the JSON output are byte for byte the same.

### Added

- **Web: step by step.** "Step by step" walks through the table one
  step at a time: each digit product (its two factors, its unit and
  its carry), each column sum, each column of every `Sub n.` section
  and of the product. The cells of each step are highlighted and
  described in English or Spanish. It has first, previous, next and
  last buttons, a section selector, and the ← and → keys on the
  table. A step takes about 2 ms even on the biggest table the web
  accepts.
- **Web: Download JSON.** The process as JSON, the same bytes as
  `long-multiplication --format json`.
- WASM: `calculateJson`, the JSON of the process with the web limits.
- `core`: `LongMultiplication::write_json`,
  `LongMultiplication::estimate_json_bytes` and `JSON_FORMAT_VERSION`
  (moved from the CLI, still without `serde`), so the CLI and the web
  share one JSON writer.
- **End-to-end test of the web page** (`scripts/e2e_web.py`, run by
  CI). It serves `web/` locally with the version stamped and the
  headers of `web/_headers`, and drives headless Chrome through
  chromedriver in English and in Spanish. It checks the 13597 x 8642
  table against its fixture, the summary line, the shared link and
  Clear, the digit, length and size errors, and a console without
  errors (Content Security Policy violations included). Only the
  Python standard library is used.

### Changed

- **Faster tables.** Runs of empty cells and the rules between rows
  are drawn once per section and written in one piece, instead of
  one character at a time. The CLI writes 200 nines times 200 nines
  (578 MB) to `/dev/null` in 0.02 s instead of 0.7 s, with the same
  memory; `get_table` is 40-88 % faster. Streaming is no longer
  slower than building the table in memory.
- Benchmarks: a `write_table in 256 KiB blocks` group measures the
  streaming path the CLI uses.
- CI: the WASM size limit is 80 KB (was 50 KB); the bundle takes
  about 56 KB with `calculateJson`, and the log shows its gzip size.

## [2.3.2] - 2026-11-24

The binary behaves as in 2.3.1; this release ships the AUR package.

### Added

- **Arch Linux package** `long-multiplication-bin` in the AUR
  (x86_64 and aarch64, the static release binary). The PKGBUILD
  lives in `packaging/aur/`; every release updates it from the
  published `SHA256SUMS`, builds and tests it with makepkg and pushes
  it to the AUR. Pull requests build it as a dry run.

### Changed

- `clap` 4.6.9.

## [2.3.1] - 2026-11-23

The text table and the JSON output are byte for byte the same.

### Changed

- **MSRV 1.88** (was 1.85). Rust has no LTS and the binaries are
  pre-built, so the minimum now follows what the dependencies need;
  the `T1 · msrv` CI job checks every target, tests included, so the
  declared version stays true. `proptest` 1.12 and `criterion` 0.8.
- `core`: `format_bytes` is public (the wording of the size errors).

### Fixed

- `get_table` and `estimate_table_bytes` follow the rules of
  `validate_input`: surrounding whitespace is ignored and an operand
  that is not made of digits gives an empty table (`0` bytes)
  instead of a made-up one.
- CLI: the `mkdir` hint quotes directories with spaces or quotes,
  and uses `mkdir "<dir>"` on Windows.
- CLI: with `--format json`, the size limit says "The JSON output
  would take about …" instead of "The table …".
- Security: `rustix` 1.1.6 (RUSTSEC-2026-0486, test-only dependency).
  `cargo audit` reports nothing.

### Removed

- `breakdown::break_down_addition`, `breakdown::break_down_multiplication`,
  `length::get_string_length`, `length::get_strings_length` and
  `length::get_numbers_length` (internal API, unused since 2.3.0;
  the first two kept 8 bytes per digit pair in memory). Their test
  cases now check `LongMultiplication::rows` and `sum`.

## [2.3.0] - 2026-11-23

The text table is byte for byte the same as before.

### Added

- **JSON output.** `--format json` (or `LONG_MULTIPLICATION_FORMAT`)
  writes every step of the process: the units and carries of each
  row, the sum of each column, every `Sub n.` section and the
  product (`format_version` 1, documented in the README and pinned
  by `tests/fixtures/*.json`). It works with `--output store|both`
  (default file `long-multiplication-output.json`) and with the size
  limits, which estimate the JSON size too.
- `core`: `LongMultiplication`, the process as data (`rows`, `sum`,
  `subtotals`, `product`), with `write_table` onto any
  `core::fmt::Write`. Only the column sums and the product are
  stored; rows and `Sub n.` sections are computed while iterating.
- Property tests with `proptest` (dev-dependency of `core`): the
  invariant test now draws random operands, including nine-heavy
  ones with several `Sub n.` sections, and shrinks any failure.
- `criterion` benchmarks for `get_table` and `estimate_table_bytes`
  (`cargo bench -p long-multiplication-core`, not run in CI).
- CI runs `cargo test --workspace` on macOS and Windows too.

### Changed

- **The CLI streams its output.** The table is written to stdout or
  the file while it is generated, so memory no longer grows with the
  table: 200 nines times 200 nines (552 MiB of text) now peak at
  about 6 MiB instead of 553 MiB. With `--output both`, stdout and
  the file are written in one pass and "Saved to …" comes after the
  table; a closed stdout pipe no longer stops the file.
- `estimate_table_bytes` no longer builds every digit product in
  memory (it used 8 bytes per digit pair), and is about 60 % faster.
- `generate`'s section functions (internal API) draw from a
  `&LongMultiplication` onto `&mut impl fmt::Write` and return
  `fmt::Result`.

## [2.2.1] - 2026-11-23

### Added

- Web: the page is bilingual. It follows the browser language
  (Spanish or English, English by default), including the error
  messages; the table keeps its byte-for-byte output.
- Web: share a calculation as a link,
  `/?multiplicand=13597&multiplier=8642`. The link is checked with
  the same rules and limits as the form and calculated on load; each
  successful calculation updates the address bar.
- Web: "Loading calculator…" until the calculator is ready, and a
  one-line summary ("Table ready: N rows.") when a table is done.
- Web: Open Graph and Twitter card tags for link previews.
- WASM: `estimateTableBytes()` is exported, so the page can explain
  the size limit in the reader's language.
- Dependabot also keeps the Rust dependencies current (one grouped
  weekly PR).

### Changed

- CLI: when `--file` points to a directory that does not exist, the
  error names it and suggests `mkdir -p <dir>`.
- `.gitattributes` keeps the reference tables in `tests/fixtures/`
  byte for byte on every platform (no CRLF conversion on Windows).
- Rustdoc: the examples in `generate.rs` use `# Examples` headings.

### Fixed

- Security: `anyhow` 1.0.105 (RUSTSEC-2026-0190). The remaining
  `rustix` advisory (RUSTSEC-2026-0486, unsound) only reaches the
  test-only `tempfile` dependency.
- Web accessibility: the table is no longer an `aria-live` region,
  so a screen reader does not read a table of megabytes aloud; the
  fields get `aria-invalid` when their value is refused.

## [2.2.0] - 2026-11-23

### Added

- **Pre-built binaries.** Every release ships the CLI for Linux
  (x86_64 and ARM64, static musl), macOS (Intel and Apple Silicon)
  and Windows (x86_64), with `SHA256SUMS` and a CycloneDX SBOM. The
  new `release.yml` builds them only from a signed tag on `main`,
  checks each binary against the golden fixture, and re-verifies the
  published assets.
- CI is split into parallel jobs (fmt, clippy, MSRV 1.85.0, tests,
  strict docs, release build with a fixture check, WASM with a size
  gate), and `cargo-audit` runs in its own workflow, also nightly.
- **Configurable limits.** `core` gains `estimate_table_bytes` (an
  upper bound of the table size, computed without rendering it) and
  `check_output_size`, plus `CoreError::OutputTooLarge`. The number
  of digits alone does not bound the table: operands made of nines
  need one `Sub n.` section per digit.
- CLI: `--max-digits` (default 1000), `--max-output-mib` (default
  1024) and `--unlimited`, also read from
  `LONG_MULTIPLICATION_MAX_DIGITS`, `LONG_MULTIPLICATION_MAX_OUTPUT_MIB`
  and `LONG_MULTIPLICATION_UNLIMITED`. `0` means no limit.
- WASM: tables over 32 MiB are refused; `maxOutputBytes()` is
  exported.
- Web: the calculation runs in a Web Worker, so big tables no longer
  freeze the page; **Clear** cancels a calculation in progress.
- Golden fixture `999999999999_x_999999999999.txt` (ten `Sub n.`
  sections) and invariant tests: every row has the same width and
  the product is correct.

### Fixed

- **Table alignment.** The `Sub 10.` (and higher) titles were one
  character too wide, and tables with more than 999 columns broke
  the position row. Cells are now wider for tables over 999 columns.
  Output for every other table is unchanged byte for byte.
- Web: the **Multiplier** field was sent as the multiplicand and
  vice versa; the fields now map correctly and the multiplicand comes
  first.
- Web: the `.wasm` binary, `app.js` and the stylesheet were cached
  for a year under unversioned or hand-written (`?v=2.0`) URLs, so
  after a release a browser could combine new JavaScript with an old
  `.wasm`. Every asset URL now carries the release version. The
  footer shows the real site version instead of `2.0`.
- Web: unknown URLs returned the page with status 200 (and a one-year
  cache under `/asset/`); `_redirects` is removed and `404.html` is
  added.
- CLI: a closed pipe (`long-multiplication A B | head`) no longer
  panics; the program ends quietly.
- CLI: `--file` with `--output display` is now an error instead of
  being silently ignored; help text is in English like the rest of
  the project.
- `CoreError::InvalidDigit::position` is documented as a character
  index (it was described as a byte offset), and `TooLong::got` now
  always counts digits (input is checked for invalid characters
  first).
- CI deploys only commits that passed CI (`workflow_run`); the CI
  workflow builds the WASM with `scripts/build_wasm.bash`, uses the
  Chrome and chromedriver already on the runner, and installs pinned
  `wasm-pack` and `cargo-audit` releases instead of `curl | sh` and
  `cargo install`.
- Docs: `DEPLOY.md` (`--no-revert`, binaryen), `ARCHITECTURE.md`
  (wrangler-action, exports, caching) and the 2.1.0 entry's claim
  that the stamp was passed between jobs as an artifact (the deploy
  job builds it itself). The Content Security Policy is now also
  sent as a header, where `frame-ancestors` takes effect.

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
- The deploy workflow uses `cloudflare/wrangler-action@v3`,
  picking up the maintained v3 line of the action.
- The web front-end footer now displays the site and app version;
  the app version is read from the WASM bundle's stamped metadata
  so visitors can confirm they are running the latest build.
- The WASM glue (`web/asset/js/wasm/`) is stamped with the
  package version at build time, so browsers stop serving the
  stale cached bundle after a new release.

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
- The WASM cache-bust stamp is propagated as a build artifact
  into the deploy job, so Cloudflare Pages always uploads a
  versioned bundle even when the deploy step runs on a fresh
  checkout.
- `actions/checkout` is bumped to v5 (Node 20 runtime) in both
  CI workflows.
- `scripts/build_wasm.sh` is renamed to `scripts/build_wasm.bash`
  to match the script-naming convention referenced in `AGENTS.md`.

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
