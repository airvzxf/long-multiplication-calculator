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
       │                 │                │    panic_hook      │
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

Public surface:

```rust
pub fn validate_input(input: &str, max_digits: usize) -> Result<&str, CoreError>;
pub fn get_table(multiplicand: &str, multiplier: &str) -> String;
pub fn display(multiplicand: &str, multiplier: &str);
pub fn store(multiplicand: &str, multiplier: &str, file_path: &str) -> std::io::Result<()>;
pub const MAX_DIGITS: usize = 1_000;
```

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
- `multiplication` — `get_table` itself, plus thin `display` and
  `store` helpers. This module is the public entry point.

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

`display` uses `println!`. `store` uses `std::fs::write`. Both
happen in the CLI binary, not in `core` (except for the tiny
re-exported `core::store` helper which is just `std::fs::write`
called on `get_table`'s output).

## Crate: `long-multiplication-wasm`

Also a thin wrapper. Two exports:

- `calculate(multiplicand: &str, multiplier: &str) -> Result<String, JsError>`
  — the actual computation. Errors are converted to `JsError` so
  the JavaScript side can `try/catch` them.
- `max_digits() -> usize` — exposed to JS for friendlier client-side
  validation.

A `#[wasm_bindgen(start)]` hook installs `console_error_panic_hook`
so that any Rust panic surfaces in the browser dev tools as a readable
message instead of an opaque abort.

## Web frontend

Pure HTML/CSS/JS, no framework. `index.html` includes a Content
Security Policy meta tag that allows `'self'` scripts and the
`'wasm-unsafe-eval'` source for WebAssembly. `app.js` is an ES module
that imports the WASM bindings, then attaches event handlers to the
form. The user never makes a network request to compute anything.

`_headers` and `_redirects` (Cloudflare Pages configuration) set
long-lived caching on the content-hashed WASM and CSS bundles, and
standard security headers on every response.

## Deployment

GitHub Actions runs CI on every push and PR. On push to `main`, a
separate workflow builds the WASM module and deploys the `web/`
directory to Cloudflare Pages via the official
`cloudflare/pages-action`. See [`docs/DEPLOY.md`](DEPLOY.md) for
the step-by-step setup.
