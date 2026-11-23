# Architecture

## High level

The project is a Cargo workspace with three crates, designed around
the **Hexagonal / Ports and Adapters** pattern. The algorithm and the
Unicode table layout live in a pure Rust library (`core`); two thin
adapters consume it: a CLI binary (`cli`) and a WebAssembly module
(`wasm`).

```
                    ┌──────────────────────────────┐
                    │   long-multiplication-core   │
                    │      (lib, no I/O)           │
                    │                              │
                    │  • error.rs  (CoreError)     │
                    │  • breakdown (algorithm)     │
                    │  • length    (digit counting)│
                    │  • generate  (Unicode)       │
                    │  • multiplication (orchestr.)│
                    └─────────────┬────────────────┘
                                  │
                ┌─────────────────┴──────────────────┐
                ▼                                    ▼
       ┌─────────────────┐                ┌────────────────────┐
       │  long-mult      │                │  long-mult-wasm    │
       │  -iplication    │                │  (cdylib)          │
       │  (bin)          │                │                    │
       │  • clap         │                │  • wasm-bindgen    │
       │  • anyhow       │                │  • console_error_  │
       │  • std::fs /    │                │    panic_hook      │
       │    println!     │                │                    │
       └────────┬────────┘                └────────┬───────────┘
                │                                   │
                ▼                                   ▼
       ┌──────────────────┐               ┌────────────────────┐
       │  stdout / file   │               │  web/asset/js/wasm/│
       │  (terminal)      │               │  + app.js          │
       └──────────────────┘               └─────────┬──────────┘
                                                    │
                                                    ▼
                                          ┌──────────────────────┐
                                          │  Cloudflare Pages    │
                                          │  (static site)       │
                                          └──────────────────────┘
```

## Crate: `long-multiplication-core`

The heart of the project. It is **deliberately boring**: no I/O, no
network, no terminal, no `serde`, no `clap`, no `wasm-bindgen`. The
only runtime dependency is `thiserror` (for the `CoreError` enum).
I/O is performed exclusively by the two adapter crates (`cli` and
`wasm`).

Public surface:

```rust
pub fn validate_input(input: &str, max_digits: usize) -> Result<&str, CoreError>;
pub fn get_table(multiplicand: &str, multiplier: &str) -> String;
pub fn estimate_table_bytes(multiplicand: &str, multiplier: &str) -> usize;
pub fn check_output_size(multiplicand: &str, multiplier: &str, max_bytes: usize) -> Result<usize, CoreError>;
pub const MAX_DIGITS: usize = 1_000; // default, not a hard limit
```

### Limits

The algorithm has no size limit of its own. The size of the table
is what matters, and the number of digits does not predict it well:
operands made of nines need one `Sub n.` section per digit, so 200
nines times 200 nines take about 550 MiB while 200 random digits take
about 20 MiB. `core` therefore offers two checks and lets each
adapter choose its limits:

- `validate_input(input, max_digits)`: digits per operand
  (`usize::MAX` = no limit).
- `check_output_size(a, b, max_bytes)`: an upper-bound estimate of
  the table size, computed without rendering it.

| Adapter | Digits | Table size | Can be lifted |
| --- | --- | --- | --- |
| CLI | 1000 | 1024 MiB | `--max-digits`, `--max-output-mib` (0 = none), `--unlimited`, or the `LONG_MULTIPLICATION_*` environment variables |
| WASM / web | 1000 | 32 MiB | No: a tab has a 4 GiB memory ceiling |

Cells are 3 characters wide; tables with more than 999 columns use
wider cells so the position row stays aligned.

The internal modules are `pub` (so doctests can reach them) but they
are not part of the supported public API:

- `breakdown` — `break_down_multiplication`, `break_down_addition`,
  `break_down_subtotal`. Pure functions over digit vectors.
- `length` — `get_string_length`, `get_strings_length`,
  `get_number_length`, `get_numbers_length`. Tells how many columns
  the table needs.
- `generate` — the per-section Unicode emitters (`symbols`,
  `top_border`, `position_title`, `operation_title`,
  `multiplication`, `operations`, `sum_title`, `long_sum`,
  `bottom_border`, `author`). These are the functions that actually
  emit the box-drawing characters. They are intentionally
  non-abstracted: there is one and only one output format (the
  school layout).
- `multiplication` — `get_table` itself, the public entry point.

### Why no `Renderer` trait?

A `Renderer` trait (with `UnicodeRenderer`, `JsonRenderer`,
`HtmlRenderer`, etc.) was considered and rejected for the v2.0.0
release. Today there is exactly one output format, and YAGNI applies:
we will not pay the maintenance cost of an abstract interface until
there is a second format to render. If and when a JSON or HTML output
is added, the trait will be extracted at that point.

### Why no `serde`?

Same reason. There is no structured data being passed between
`core` and the outside world. If the web frontend eventually needs
to highlight each step, the structured data will be added then and
`serde` will be a feature-gated dependency of `core`.

## Crate: `long-multiplication` (the CLI)

A thin wrapper over `core`. The `main.rs` is intentionally small:

1. Parse with `clap` (derive API).
2. Validate each argument via `core::validate_input` (returns
   `CoreError` which `anyhow` wraps for context).
3. Call `core::get_table` once.
4. Dispatch on `--output` (`display`, `store`, `both`).

The CLI is the only place that does I/O for this project: the
`display` branch calls `println!`, and the `store` and `both`
branches call `std::fs::write` on a private `write_table` helper.
Keeping I/O in the adapter (not in `core`) is what lets `core` be
embedded in `wasm32` without dragging in a filesystem or terminal.

## Crate: `long-multiplication-wasm`

Also a thin wrapper. Exports:

- `calculate(multiplicand: &str, multiplier: &str) -> Result<String, JsError>`
  — the actual computation, after checking the digit and size
  limits. Errors are converted to `JsError` so the JavaScript side
  can `try/catch` them.
- `maxDigits()` and `maxOutputBytes()` — the limits, for friendlier
  client-side validation.
- `version()` — the package version, shown in the page footer.

A `#[wasm_bindgen(start)]` hook installs `console_error_panic_hook`
so that any Rust panic surfaces in the browser dev tools as a readable
message instead of an opaque abort.

## Web frontend

Pure HTML/CSS/JS, no framework. `app.js` handles the form and runs
every calculation in a module Web Worker (`worker.js`) that loads
the WASM bindings, so a big table never freezes the page and
**Clear** cancels it. The user never makes a network request to
compute anything.

The interface is English, or Spanish when the browser prefers it:
`i18n.js` holds both dictionaries and translates every
`data-i18n` element. The worker reports a refused table as numbers
(`code: "too-large"`, the estimate and the limit, from the
`estimateTableBytes` export) so the page can word it in either
language; the table itself is the contractual output and stays as
it is. A link such as `/?multiplicand=13597&multiplier=8642` fills
the form and calculates it with the same checks and limits, and
each successful calculation writes its link to the address bar.
Only a one-line summary (`Table ready: N rows.`) is announced to
screen readers, never the table.

`_headers` (Cloudflare Pages configuration) sends the Content
Security Policy (`'self'` scripts plus `'wasm-unsafe-eval'` for
WebAssembly; `index.html` repeats it in a meta tag for local
servers), standard security headers, and a one-year `immutable`
cache for `/asset/*`. That cache is safe because every asset URL
carries the release version (`?v=<version>`), stamped by
`scripts/build_wasm.bash`. `404.html` gives unknown URLs a real 404
instead of the page.

## Deployment

GitHub Actions runs CI on every push and PR. When CI succeeds on
`main`, a separate workflow builds the WASM module and deploys the
`web/` directory to Cloudflare Pages via the official
`cloudflare/wrangler-action`. See [`docs/DEPLOY.md`](DEPLOY.md) for
the step-by-step setup.
