# Long Multiplication Calculator

Multi-target long multiplication, step by step, the way you learned it
in primary school. Rendered with Unicode box-drawing characters so the
table is readable in any terminal and on any device.

![Rust 2024](https://img.shields.io/badge/rust-2024-orange) ![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue)

## Try it

- **Web (no install):** <https://multiplication.rovisoft.net>
- **CLI:** download a binary from the [releases](https://github.com/airvzxf/long-multiplication-calculator/releases/latest), or `cargo install --path crates/cli`
- **As a library:** see [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)

## Features

- **100% static web** — computation runs in the browser via WebAssembly.
  No backend, no server, no third-party scripts.
- **Identical CLI output** — `long-multiplication A B` produces the
  exact same Unicode table as the v1.0.0 binary, byte for byte.
- **Centralised validation** — both the CLI and the WASM build use
  the same `validate_input` function from `core`.
- **Workspace layout** — `core` (algorithm), `cli` (binary), `wasm`
  (browser bindings). Reuse `core` from your own Rust project.

## Installation

### CLI: pre-built binary (no Rust needed)

Download the archive for your system from the
[latest release](https://github.com/airvzxf/long-multiplication-calculator/releases/latest):

| System | Archive |
| --- | --- |
| Linux x86_64 (any distro) | `long-multiplication-vX.Y.Z-x86_64-unknown-linux-musl.tar.gz` |
| Linux ARM64 (any distro) | `long-multiplication-vX.Y.Z-aarch64-unknown-linux-musl.tar.gz` |
| macOS Apple Silicon | `long-multiplication-vX.Y.Z-aarch64-apple-darwin.tar.gz` |
| macOS Intel | `long-multiplication-vX.Y.Z-x86_64-apple-darwin.tar.gz` |
| Windows x86_64 | `long-multiplication-vX.Y.Z-x86_64-pc-windows-msvc.zip` |

```bash
sha256sum -c SHA256SUMS --ignore-missing      # optional: verify the download
tar -xzf long-multiplication-*-x86_64-unknown-linux-musl.tar.gz
./long-multiplication-*/long-multiplication 13597 8642
```

The binaries are not code-signed. On macOS, run
`xattr -d com.apple.quarantine long-multiplication` once; on Windows,
choose "More info → Run anyway" if SmartScreen asks.

### CLI: from source

```bash
cargo install --path crates/cli
long-multiplication --help
```

### From source (web)

```bash
git clone https://github.com/airvzxf/long-multiplication-calculator
cd long-multiplication-calculator
./scripts/build_wasm.bash
cd web
python3 -m http.server 8000
# open http://localhost:8000
```

## Quick examples

```bash
# Display the table for 13597 * 8642
long-multiplication 13597 8642

# Save the table to a file
long-multiplication 5 7 --output store --file result.txt

# Show both stdout and file
long-multiplication 99 99 --output both
```

## Limits

The table grows fast: two 200-digit numbers made of nines produce
about 550 MiB of text. To keep that from happening by accident:

| | Digits per number | Table size |
| --- | --- | --- |
| Web | 1000 | 32 MiB (fixed) |
| CLI | 1000 | 1024 MiB |

The CLI limits are only defaults. Change them per run or for your
whole session:

```bash
long-multiplication --max-digits 5000 --max-output-mib 4096 A B   # raise them
long-multiplication --unlimited A B > table.txt                   # remove them
export LONG_MULTIPLICATION_UNLIMITED=true                          # same, for every run
```

`0` also means "no limit" for `--max-digits` / `--max-output-mib`
(and for `LONG_MULTIPLICATION_MAX_DIGITS` /
`LONG_MULTIPLICATION_MAX_OUTPUT_MIB`). Without limits, only your
memory, disk and time are the limit.

## Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

The full project protocol ("The Gauntlet") is documented in
[`AGENTS.md`](AGENTS.md).

## Repository layout

```
crates/
  core/   # pure algorithm + Unicode table generator
  cli/    # `long-multiplication` binary
  wasm/   # browser bindings (wasm-bindgen)
web/      # static site served by Cloudflare Pages
scripts/  # helper shell scripts (e.g. build_wasm.bash)
.github/  # CI and CD workflows
docs/     # ARCHITECTURE.md, DEPLOY.md, TESTING.md
```

## License

AGPL-3.0. See [LICENSE](LICENSE).
