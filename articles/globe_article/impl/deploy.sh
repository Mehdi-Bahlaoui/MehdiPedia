#!/usr/bin/env bash
# Build the Rust/WebGL2 globe, copy it into the site's vendor directory, and
# point the page's preloads + inline loader at the new cache version.
#
#   ./deploy.sh        # keep the current ?v=2 (only safe if the wasm is unchanged)
#   ./deploy.sh 3      # use ?v=3 (do this whenever the wasm changes)
set -euo pipefail

cd "$(dirname "$0")"

VERSION="${1:-2}"
if ! [[ "$VERSION" =~ ^[0-9]+$ ]]; then
    echo "version must be an integer, e.g. ./deploy.sh 3" >&2
    exit 1
fi
if [ $# -eq 0 ]; then
    echo "No version given; keeping ?v=${VERSION}. Pass a new number after a wasm change: ./deploy.sh 3"
fi

trunk build --release

JS_SRC=$(ls dist/globe-web.js 2>/dev/null || ls dist/*.js | head -n 1)
WASM_SRC=$(ls dist/globe-web_bg.wasm 2>/dev/null || ls dist/*_bg.wasm | head -n 1)

DEST="../../globe-vendor"
HTML="../../globe.html"

mkdir -p "$DEST"
cp "$JS_SRC" "$DEST/globe_app.js"
cp "$WASM_SRC" "$DEST/globe_app.wasm"

# Keep the two preloads and the two inline-loader URLs on the same query.
sed -i -E "s/\\?v=[0-9]+/?v=${VERSION}/g" "$HTML"

echo "--- deployed ---"
ls -lh "$DEST/globe_app.js" "$DEST/globe_app.wasm"
echo "--- gzip transfer sizes ---"
gzip -9 -c "$DEST/globe_app.wasm" | wc -c
gzip -9 -c "$DEST/globe_app.js" | wc -c
echo "--- globe.html version ---"
grep -oE '\?v=[0-9]+' "$HTML" | sort -u
