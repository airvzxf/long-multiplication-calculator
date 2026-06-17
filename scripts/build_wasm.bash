#!/usr/bin/env bash
#
# Build the long-multiplication WASM module and copy it into the
# static web/ directory so Cloudflare Pages can serve it.
#
# Two phases:
#   1. wasm-pack builds the .wasm + JS glue (with --no-opt).
#   2. We run wasm-opt ourselves with --enable-bulk-memory and
#      --enable-sign-ext, because recent Rust toolchains emit WASM
#      that uses `memory.copy` and sign-extension ops (i32.extend8_s,
#      etc.) and wasm-pack's default `-O` invocation does not enable
#      these features, so it fails validation.
#
# A third phase stamps the WASM package version (read from
# Cargo.toml) as a `?v=<version>` cache-bust suffix on the two
# files that reference the WASM glue: `web/asset/js/app.js` (the
# import) and `web/index.html` (the modulepreload). The committed
# source has the placeholder `?v=__WASM_VERSION__`; this script is
# the only place that resolves it. The substitution is reverted on
# EXIT so the working tree stays clean after a local build, and
# the deploy workflow picks up the stamped files from the CI
# runner's filesystem before wrangler uploads `web/`.
#
# Usage:
#   ./scripts/build_wasm.bash         # optimised bundle (~25 KB)
#   ./scripts/build_wasm.bash --no-opt  # skip wasm-opt (~45 KB)
#
# Requirements:
#   * Rust with the `wasm32-unknown-unknown` target installed:
#       rustup target add wasm32-unknown-unknown
#   * wasm-pack: https://rustwasm.github.io/wasm-pack/
#   * wasm-opt (binaryen, optional): https://github.com/WebAssembly/binaryen

set -euo pipefail

SKIP_OPT=0
if [[ "${1:-}" == "--no-opt" ]]; then
    SKIP_OPT=1
fi

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${REPO_ROOT}/web/asset/js/wasm"
WASM_FILE="${OUT_DIR}/long_multiplication_wasm_bg.wasm"

# Read the workspace version from Cargo.toml. This becomes the
# `?v=<version>` cache-bust suffix on the WASM glue references,
# so the browser always re-fetches the glue (and its modulepreload)
# whenever the package version changes.
WASM_VERSION=$(grep -E '^version = ' "${REPO_ROOT}/Cargo.toml" | head -1 | sed -E 's/^version = "(.*)"/\1/')
if [[ -z "${WASM_VERSION}" ]]; then
    echo ">>> ERROR: could not read version from ${REPO_ROOT}/Cargo.toml" >&2
    exit 1
fi
echo ">>> WASM version: ${WASM_VERSION}"

echo ">>> Building WASM (target: web, release, out: ${OUT_DIR})"
wasm-pack build \
    "${REPO_ROOT}/crates/wasm" \
    --target web \
    --release \
    --out-dir "${OUT_DIR}" \
    --no-opt

if [[ "${SKIP_OPT}" -eq 1 ]]; then
    echo ">>> Skipping wasm-opt (--no-opt)"
elif command -v wasm-opt >/dev/null 2>&1; then
    echo ">>> Optimising with wasm-opt --enable-bulk-memory --enable-sign-ext -Oz"
    wasm-opt \
        --enable-bulk-memory \
        --enable-sign-ext \
        -Oz \
        "${WASM_FILE}" \
        -o "${WASM_FILE}.opt"
    mv "${WASM_FILE}.opt" "${WASM_FILE}"
else
    echo ">>> wasm-opt not found on \$PATH; bundle left unoptimised"
fi

# Stamp the cache-bust suffix into the two files that reference
# the WASM glue. The committed source has `?v=__WASM_VERSION__`;
# this is the only place it gets resolved to a real version.
echo ">>> Stamping cache-bust: ?v=${WASM_VERSION}"
sed -i "s|?v=__WASM_VERSION__|?v=${WASM_VERSION}|g" \
    "${REPO_ROOT}/web/asset/js/app.js" \
    "${REPO_ROOT}/web/index.html"

# Always revert the stamp at exit (success or failure) so the
# working tree is left in the committed state. The deploy runner
# reads the stamped files before this trap fires, so `web/` is
# still uploaded with the version baked in.
trap 'sed -i "s|?v='"${WASM_VERSION}"'|?v=__WASM_VERSION__|g" \
    "${REPO_ROOT}/web/asset/js/app.js" \
    "${REPO_ROOT}/web/index.html"' EXIT

echo ">>> Done. Files:"
ls -lh "${OUT_DIR}"
