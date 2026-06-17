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
# the only place that resolves it.
#
# By default, the stamp is reverted on EXIT so the working tree
# stays clean after a local build. Pass `--no-revert` to keep the
# stamp in place; the deploy workflow (CI) uses this so wrangler
# uploads the version-baked files instead of the placeholder.
#
# Usage:
#   ./scripts/build_wasm.bash              # optimised, reverts stamp
#   ./scripts/build_wasm.bash --no-opt     # skip wasm-opt
#   ./scripts/build_wasm.bash --no-revert  # keep the stamp (CI)
#
# Requirements:
#   * Rust with the `wasm32-unknown-unknown` target installed:
#       rustup target add wasm32-unknown-unknown
#   * wasm-pack: https://rustwasm.github.io/wasm-pack/
#   * wasm-opt (binaryen, optional): https://github.com/WebAssembly/binaryen

set -euo pipefail

SKIP_OPT=0
NO_REVERT=0
for arg in "$@"; do
    case "$arg" in
        --no-opt) SKIP_OPT=1 ;;
        --no-revert) NO_REVERT=1 ;;
        -h|--help)
            sed -n '2,35p' "${BASH_SOURCE[0]}"
            exit 0
            ;;
        *) echo ">>> ERROR: unknown argument: $arg" >&2; exit 1 ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${REPO_ROOT}/web/asset/js/wasm"
WASM_FILE="${OUT_DIR}/long_multiplication_wasm_bg.wasm"
STAMP_FILES=(
    "${REPO_ROOT}/web/asset/js/app.js"
    "${REPO_ROOT}/web/index.html"
)

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
sed -i "s|?v=__WASM_VERSION__|?v=${WASM_VERSION}|g" "${STAMP_FILES[@]}"

# By default, revert the stamp at exit (success or failure) so the
# working tree is left in the committed state for local builds.
# `--no-revert` skips the trap so the stamped files survive until
# the deploy step (wrangler pages deploy) reads them.
if [[ "${NO_REVERT}" -eq 0 ]]; then
    trap 'sed -i "s|?v='"${WASM_VERSION}"'|?v=__WASM_VERSION__|g" "${STAMP_FILES[@]}"' EXIT
    echo ">>> Stamp will be reverted on EXIT (use --no-revert to keep it)"
else
    echo ">>> Stamp will be kept (--no-revert); files are ready for deploy"
fi

echo ">>> Done. Files:"
ls -lh "${OUT_DIR}"
