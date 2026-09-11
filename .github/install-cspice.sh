#!/usr/bin/env bash
#
# Install the NAIF CSPICE toolkit into the directory given as the first argument, ready to be
# pointed at by CSPICE_DIR.
#
#     .github/install-cspice.sh ~/cspice
#
# NAIF ships a prebuilt static library per platform. The only thing that needs fixing up is its
# name: the archive holds `cspice.a`, and Unix linkers look for `libcspice.a`.
#
# Set CSPICE_BASE_URL to fetch from a mirror, or from a local copy, instead of from NAIF.

set -euo pipefail

destination="${1:?usage: install-cspice.sh <destination>}"
# Overridable so the script can be pointed at a mirror, or at a local copy for testing.
base="${CSPICE_BASE_URL:-https://naif.jpl.nasa.gov/pub/naif/toolkit/C}"

case "$(uname -s)/$(uname -m)" in
  Linux/x86_64)  package="PC_Linux_GCC_64bit" ;;
  Linux/i?86)    package="PC_Linux_GCC_32bit" ;;
  Darwin/arm64)  package="MacM1_OSX_clang_64bit" ;;
  Darwin/x86_64) package="MacIntel_OSX_AppleC_64bit" ;;
  *)
    echo "no CSPICE package published for $(uname -s)/$(uname -m)" >&2
    echo "see ${base}/ for the full list" >&2
    exit 1
    ;;
esac

if [ -e "$destination" ]; then
  echo "$destination already exists; nothing to do" >&2
  exit 0
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT

echo "Downloading $package"
curl --fail --location --silent --show-error \
  "${base}/${package}/packages/cspice.tar.Z" --output "${work}/cspice.tar.Z"

gzip --decompress "${work}/cspice.tar.Z"
tar --extract --file "${work}/cspice.tar" --directory "$work"

mv "${work}/cspice/lib/cspice.a" "${work}/cspice/lib/libcspice.a"

mkdir -p "$(dirname "$destination")"
mv "${work}/cspice" "$destination"

echo "CSPICE installed in $destination"
