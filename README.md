# Long Multiplication Calculator

Multi-target long multiplication, step by step, the way you learned it
in primary school. Rendered with Unicode box-drawing characters so the
table is readable in any terminal and on any device.

![Rust 2024](https://img.shields.io/badge/rust-2024-orange) ![License: AGPL-3.0](https://img.shields.io/badge/License-AGPL--3.0-blue)

## Try it

- **Web (no install):** <https://multiplication.rovisoft.net>
- **CLI:** `cargo install --path crates/cli` then `long-multiplication 13597 8642`
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

### CLI

```bash
cargo install --path crates/cli
long-multiplication --help
```

### From source (web)

```bash
git clone https://github.com/airvzxf/long-multiplication-calculator
cd long-multiplication-calculator
./scripts/build_wasm.sh
cd web
python3 -m http.server 8000
# open http://localhost:8000
```

## Quick examples

```bash
# Display the table for 13597 * 8642
long-multiplication 13597 8642

# Save the table to a file
long-multiplication 5 7 --output store --file resultado.txt

# Show both stdout and file
long-multiplication 99 99 --output both
```

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
scripts/  # helper shell scripts (e.g. build_wasm.sh)
.github/  # CI and CD workflows
docs/     # ARCHITECTURE.md, DEPLOY.md, TESTING.md
```

## License

AGPL-3.0. See [LICENSE](LICENSE).
