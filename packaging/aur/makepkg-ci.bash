#!/usr/bin/env bash
#
# Build packaging/aur/PKGBUILD in CI (an archlinux:base-devel
# container running as root): makepkg refuses to run as root, so the
# build runs as a throwaway user in /tmp/aur-build. It downloads the
# release archive, checks its checksum, runs check() (version and a
# golden row) and packages it; it also writes /tmp/aur-build/.SRCINFO.

set -euo pipefail

SRC="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD=/tmp/aur-build

id builder >/dev/null 2>&1 || useradd --create-home builder
rm -rf "${BUILD}"
install -d -o builder "${BUILD}"
install -o builder -m644 "${SRC}/PKGBUILD" "${BUILD}/PKGBUILD"

su builder -c "cd ${BUILD} && makepkg --noconfirm --cleanbuild && makepkg --printsrcinfo > .SRCINFO"
ls -la "${BUILD}"
