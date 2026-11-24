#!/usr/bin/env bash
#
# Point packaging/aur/PKGBUILD at a published release.
#
# Sets `pkgver` to VERSION, resets `pkgrel` to 1 and copies the
# checksums of the two Linux archives from the release's SHA256SUMS,
# so the PKGBUILD never trusts a checksum it computed itself.
#
# Usage:
#   packaging/aur/update.bash 2.3.1 path/to/SHA256SUMS
#   packaging/aur/update.bash 2.3.1          # downloads SHA256SUMS
#
# Then, in packaging/aur/: `makepkg --printsrcinfo > .SRCINFO`.

set -euo pipefail

VERSION="${1:?usage: update.bash VERSION [SHA256SUMS]}"
SUMS="${2:-}"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PKGBUILD="${DIR}/PKGBUILD"
REPO="https://github.com/airvzxf/long-multiplication-calculator"

if [[ ! "${VERSION}" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "ERROR: '${VERSION}' is not a release version (X.Y.Z)" >&2
    exit 1
fi

if [[ -z "${SUMS}" ]]; then
    SUMS="$(mktemp)"
    trap 'rm -f "${SUMS}"' EXIT
    curl --fail --silent --show-error --location \
        "${REPO}/releases/download/v${VERSION}/SHA256SUMS" -o "${SUMS}"
fi

sum_of() {
    local file="long-multiplication-v${VERSION}-$1-unknown-linux-musl.tar.gz"
    local sum
    sum="$(awk -v f="${file}" '$2 == f { print $1 }' "${SUMS}")"
    if [[ ! "${sum}" =~ ^[0-9a-f]{64}$ ]]; then
        echo "ERROR: no checksum for ${file} in SHA256SUMS" >&2
        exit 1
    fi
    echo "${sum}"
}

X86_64="$(sum_of x86_64)"
AARCH64="$(sum_of aarch64)"

sed -i -E \
    -e "s/^pkgver=.*/pkgver=${VERSION}/" \
    -e "s/^pkgrel=.*/pkgrel=1/" \
    -e "s/^sha256sums_x86_64=.*/sha256sums_x86_64=('${X86_64}')/" \
    -e "s/^sha256sums_aarch64=.*/sha256sums_aarch64=('${AARCH64}')/" \
    "${PKGBUILD}"

grep -E '^(pkgver|pkgrel|sha256sums_)' "${PKGBUILD}"
