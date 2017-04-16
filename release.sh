#!/bin/sh

set -euo pipefail

VERSION=$(cargo pkgid | grep -o '#.*' | cut -b '2-')

cargo +stable build --release
cp target/release/lproj2es .   # needs to copy due to the hardlink.
xz -6 -e -S "-${VERSION}.xz" lproj2es
