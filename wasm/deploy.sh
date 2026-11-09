#!/usr/bin/env bash
# Build a Rust/WebGL2 scene from wasm/scenes/<scene>, copy its bundle into
# articles/<scene>/wasm/, and update the page's ?v= cache-buster when mapped.
#
# Usage:
#   ./deploy.sh                  # build globe, keep its current ?v=
#   ./deploy.sh 3                # shorthand for: ./deploy.sh globe 3
#   ./deploy.sh globe 3          # build globe and set ?v=3 in articles/globe/index.html
#   ./deploy.sh confetti         # build confetti into articles/confetti/wasm/
set -euo pipefail

cd "$(dirname "$0")"

SCENE="globe"
VERSION=""

if [ $# -ge 1 ]; then
    if [[ "$1" =~ ^[0-9]+$ ]]; then
        VERSION="$1"
    else
        SCENE="$1"
        VERSION="${2:-}"
    fi
fi

SCENE_DIR="scenes/${SCENE}"
if [ ! -d "$SCENE_DIR" ]; then
    echo "Unknown scene '${SCENE}' (expected directory wasm/${SCENE_DIR})" >&2
    exit 1
fi

HTML=""
case "$SCENE" in
    globe)
        HTML="../articles/globe/index.html"
        ;;
    confetti)
        HTML="../articles/confetti/index.html"
        ;;
esac

if [ -n "$HTML" ] && [ -z "$VERSION" ]; then
    VERSION=$(grep -oE '\?v=[0-9]+' "$HTML" | head -n 1 | cut -d= -f2 || echo "1")
    echo "No version given; keeping ?v=${VERSION}. Pass a new number after a wasm change: ./deploy.sh ${SCENE} $((VERSION + 1))"
fi

if [ -n "$VERSION" ] && ! [[ "$VERSION" =~ ^[0-9]+$ ]]; then
    echo "version must be an integer, e.g. ./deploy.sh ${SCENE} 3" >&2
    exit 1
fi

(cd "$SCENE_DIR" && trunk build --release)

JS_SRC=$(ls "${SCENE_DIR}/dist/${SCENE}.js" 2>/dev/null || ls "${SCENE_DIR}"/dist/*.js | head -n 1)
WASM_SRC=$(ls "${SCENE_DIR}/dist/${SCENE}_bg.wasm" 2>/dev/null || ls "${SCENE_DIR}"/dist/*_bg.wasm | head -n 1)

DEST="../articles/${SCENE}/wasm"
mkdir -p "$DEST"
cp "$JS_SRC" "$DEST/${SCENE}_app.js"
cp "$WASM_SRC" "$DEST/${SCENE}_app.wasm"

if [ -n "$HTML" ] && [ -n "$VERSION" ]; then
    sed -i -E "s/\\?v=[0-9]+/?v=${VERSION}/g" "$HTML"
fi

echo "--- deployed (${SCENE}) ---"
ls -lh "$DEST/${SCENE}_app.js" "$DEST/${SCENE}_app.wasm"
echo "--- gzip transfer sizes ---"
printf "wasm: %s bytes\n" "$(gzip -9 -c "$DEST/${SCENE}_app.wasm" | wc -c)"
printf "js:   %s bytes\n" "$(gzip -9 -c "$DEST/${SCENE}_app.js" | wc -c)"

if [ -n "$HTML" ]; then
    echo "--- ${HTML#../} version ---"
    grep -oE '\?v=[0-9]+' "$HTML" | sort -u
fi
