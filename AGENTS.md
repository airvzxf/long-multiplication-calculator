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
./scripts/build_wasm.bash      # build + wasm-opt -Oz into web/asset/js/wasm/
wasm-pack test --chrome --headless crates/wasm   # or --firefox --headless
```

The CI workflow (`.github/workflows/ci.yml`) runs the same checks on
every push, and the deploy workflow only runs after CI passes on
`main`. Locally you can skip `cargo audit` and the WASM build if
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
`1.88.0` (enforced via `.clippy.toml`, `package.rust-version` and the
`T1 · msrv` CI job, which checks every target, tests included).

The MSRV is not a promise to old compilers: Rust has no LTS, and
users get pre-built binaries or the current stable through
`rust-toolchain.toml`. When a dependency or a new language feature
needs a newer Rust, raise `rust-version`, `.clippy.toml` and the CI
job together, to the lowest version that works.
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
  the golden reference, and so is the JSON layout of
  `--format json` (`format_version` 1, `tests/fixtures/*.json`). If
  you change the algorithm and the output changes, document why in
  `CHANGELOG.md` and consider a major version bump.
- **No `unwrap()` or `expect()` in production code** (i.e. anything
  that is not `#[cfg(test)]` or a doctest). Use `Result<T, CoreError>`
  in `core` and `anyhow::Result` in `cli`.

## Release checklist

Nothing is published to crates.io; a release is a GitHub Release
with pre-built CLI binaries (`.github/workflows/release.yml`).

1. In a PR: bump `version` in the root `Cargo.toml` (and
   `Cargo.lock`), and turn `## [Unreleased]` in `CHANGELOG.md` into
   `## [X.Y.Z] - YYYY-MM-DD` (the release notes are taken from that
   section; the workflow fails without it).
2. Wait for CI on the PR. It also runs a release **dry run** (the
   five-target build matrix, nothing published) when the PR touches
   the CLI, `core`, manifests or the release workflow.
3. Squash-merge. Wait for CI and the Cloudflare deploy on `main`.
4. Tag **the squash commit on main** with a **signed** tag (the
   workflow rejects tags that are unsigned, signed by a key missing
   from `.github/trusted-signers`, or not reachable from `main`):

   ```bash
   git fetch origin
   git tag -s vX.Y.Z "$(git rev-parse origin/main)" -m "release: X.Y.Z"
   git push origin vX.Y.Z
   ```

5. Wait for the `release` workflow: it checks the tag, builds and
   smoke-tests five targets, publishes the Release with
   `SHA256SUMS` and an SBOM, then downloads the published assets
   and verifies them (`Verify · published assets`). Finally it
   updates the AUR package `long-multiplication-bin`
   (`Publish · AUR`): `packaging/aur/update.bash` takes the version
   and the published checksums, makepkg builds and tests it, and
   PKGBUILD + .SRCINFO are pushed with the `AUR_SSH_PRIVATE_KEY`
   secret. Nothing to do by hand; the copy in `packaging/aur/` can be
   synced in the next release PR (`update.bash X.Y.Z`, then
   `makepkg --printsrcinfo > .SRCINFO`).
6. If it fails after the tag exists, fix on `main` and either
   re-run it (`Actions → release → Run workflow`, tag `vX.Y.Z`) or
   delete and re-create the tag on the new commit.

## Common pitfalls

- **WASM target missing:** `rustup target add wasm32-unknown-unknown`.
- **`wasm-pack` not found:** install from
  <https://rustwasm.github.io/wasm-pack/>.
- **`wasm-opt` aborts with `memory.copy operations require bulk memory operations`:** the
  Rust toolchain emits WASM that uses the bulk-memory proposal. Pass
  `--enable-bulk-memory` to `wasm-opt` (the `scripts/build_wasm.bash`
  script does this for you). wasm-pack's default `-O` invocation
  does not enable bulk-memory, so do **not** let wasm-pack call
  wasm-opt on its own — use `--no-opt` and run `wasm-opt` yourself.
- **`wasm-pack test` fails with `failed to find the
  __wbindgen_externref_table_alloc function`:** a global
  `build.rustflags = ["-C", "target-cpu=native"]` (for example in
  `~/.cargo/config.toml`) also applies to `wasm32`. Run the WASM
  commands with `RUSTFLAGS=""`, which overrides it.
- **Clippy lints failing:** see the per-crate `[lints.clippy]`
  sections in `Cargo.toml`. The project allows a small set of
  `clippy::all` lints that match the original 2021 code style
  (e.g. `needless_late_init`, `single_char_add_str`).
- **`cargo audit` failing on an existing dependency:** discuss before
  upgrading; SemVer 2.0.0 will be bumped if a fix requires a major
  version of a transitive dep.
