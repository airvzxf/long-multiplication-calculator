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
# A third phase stamps the web sources:
#   * `__ASSET_VERSION__`, the `?v=` cache-bust suffix of every asset
#     URL (CSS, app.js, i18n.js, steps.js, worker.js, the WASM glue and
#     the `.wasm` binary), becomes `<version>-<commit>` (for example
#     `2.4.0-15e5355a1b2c`), or just the version outside a git
#     checkout;
#   * `__WASM_VERSION__`, the site version shown in the footer, becomes
#     the package version (read from Cargo.toml).
# Because every deployed commit gets new asset URLs, `web/_headers`
# can cache /asset/* for a year without ever serving a stale file,
# even when `main` is deployed between two releases. (`-`, not `+`:
# a `+` in a query string reads as a space.)
#
# By default, the stamped files are restored on EXIT so the working
# tree stays clean after a local build. Pass `--no-revert` to keep
# the stamp in place; the deploy workflow uses this so wrangler
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
            sed -n '2,36p' "${BASH_SOURCE[0]}"
            exit 0
            ;;
        *) echo ">>> ERROR: unknown argument: $arg" >&2; exit 1 ;;
    esac
done

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT_DIR="${REPO_ROOT}/web/asset/js/wasm"
WASM_FILE="${OUT_DIR}/long_multiplication_wasm_bg.wasm"
STAMP_FILES=(
    "${REPO_ROOT}/web/index.html"
    "${REPO_ROOT}/web/asset/js/app.js"
    "${REPO_ROOT}/web/asset/js/i18n.js"
    "${REPO_ROOT}/web/asset/js/steps.js"
    "${REPO_ROOT}/web/asset/js/worker.js"
)

# Read the workspace version from Cargo.toml: the footer shows it,
# and it starts the `?v=` cache-bust suffix of every asset URL.
WASM_VERSION=$(grep -E '^version = ' "${REPO_ROOT}/Cargo.toml" | head -1 | sed -E 's/^version = "(.*)"/\1/')
if [[ -z "${WASM_VERSION}" ]]; then
    echo ">>> ERROR: could not read version from ${REPO_ROOT}/Cargo.toml" >&2
    exit 1
fi
echo ">>> WASM version: ${WASM_VERSION}"

# The asset URLs change with every commit, not only with every
# release: the edge and the browsers cache /asset/* as immutable.
if COMMIT=$(git -C "${REPO_ROOT}" rev-parse --short=12 HEAD 2>/dev/null); then
    ASSET_VERSION="${WASM_VERSION}-${COMMIT}"
else
    ASSET_VERSION="${WASM_VERSION}"
fi
echo ">>> Asset version: ${ASSET_VERSION}"

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

# Stamp the version into the web sources. Keep a pristine copy first
# so a local build can restore the committed placeholders on EXIT,
# whether the build succeeds or fails.
if [[ "${NO_REVERT}" -eq 0 ]]; then
    BACKUP_DIR="$(mktemp -d)"
    for file in "${STAMP_FILES[@]}"; do
        cp -p "${file}" "${BACKUP_DIR}/$(basename "${file}")"
    done
    restore_stamp() {
        for file in "${STAMP_FILES[@]}"; do
            cp -p "${BACKUP_DIR}/$(basename "${file}")" "${file}"
        done
        rm -rf "${BACKUP_DIR}"
    }
    trap restore_stamp EXIT
    echo ">>> Stamp will be reverted on EXIT (use --no-revert to keep it)"
else
    echo ">>> Stamp will be kept (--no-revert); files are ready for deploy"
fi

echo ">>> Stamping version: ${WASM_VERSION}"
sed -i -e "s|__ASSET_VERSION__|${ASSET_VERSION}|g" -e "s|__WASM_VERSION__|${WASM_VERSION}|g" "${STAMP_FILES[@]}"
if grep -lE "__(ASSET|WASM)_VERSION__" "${STAMP_FILES[@]}"; then
    echo ">>> ERROR: placeholder left in the files above" >&2
    exit 1
fi

echo ">>> Done. Files:"
ls -lh "${OUT_DIR}"
