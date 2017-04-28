#!/bin/sh

set -euo pipefail

VERSION=$(cargo pkgid | grep -o '#.*' | cut -b '2-')
PLATFORM="$(uname -s)-$(uname -m)"

cargo +stable build --release
cd target/release
tar cfJ "../../lproj2es-${VERSION}-${PLATFORM}.tar.xz" lproj2es lproj2es-server
