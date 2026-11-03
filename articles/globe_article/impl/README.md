# Dot Globe — Rust + WebGL2 (no Bevy)

A small, drop-in replacement for the Bevy globe in `../../../../Globe_Rust/`.
It renders the same scene directly with WebGL2:

- 9,251 baked land dots (one `gl.POINTS` draw call)
- Paris / New York / Tokyo markers
- Three animated great-circle routes with a pulsing tail
- Drag / touch to orbit, slow auto-rotation
- Same `#globe-container` / `.globe-layout` contract as the old build, including
  the first-frame signal and the slide-in classes

No Bevy, no runtime assets, no shader/pipeline compilation zoo.

## Why this exists

| | Bevy build | This build |
|---|---|---|
| WASM | 9.4 MB (3.6 MB gzip) | **~156 KB (~123 KB gzip)** |
| JS glue | 97 KB | **~23 KB (~5 KB gzip)** |
| Startup work | ~1.6 s of pipeline/shader setup after download | three tiny shader programs + small buffer uploads |
| Dot geometry | 9,251 × 42-vertex spheres ≈ 388k verts / 740k tris / ~18 MB of buffers | 9,251 point sprites, ~111 KB of positions |
| Runtime assets | none (baked land data) | none (baked land data) |

The Bevy version’s download and main-thread initialization were the reason the
globe still felt slow even with the prefetch/hover-warm tricks in `assets/js/script.js`.

Cold-cache first frame (Chrome + SwiftShader, CDP throttling, cache disabled):

| Network | This build | Live Bevy build |
|---|---|---|
| 4G (9 Mbps, 50 ms) | **0.52 s** | 5.05 s |
| Fast 3G (1.6 Mbps, 150 ms) | **2.35 s** | 21.25 s |

The local test server sends the wasm uncompressed (159 KB); GitHub Pages sends
123 KB gzip, so the deployed numbers are slightly better.

## Build

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk        # if you don't have it

trunk serve                # dev at http://127.0.0.1:8080
trunk build --release      # production in dist/
```

The release profile is size-first (`opt-level = "z"`, fat LTO, `panic = "abort"`),
and Trunk runs `wasm-opt -Oz` (`data-wasm-opt="z"` in `index.html`).

`wasm-bindgen` is pinned to `0.2.108` so Trunk can reuse the CLI version it
already caches. If you bump it, let Trunk download the matching CLI.

## Deploy into the site

```bash
./deploy.sh 2
```

`./deploy.sh` builds, copies:

- `dist/globe-web.js`  → `articles/globe-vendor/globe_app.js`
- `dist/globe-web_bg.wasm` → `articles/globe-vendor/globe_app.wasm`

and bumps the `?v=` query on the two preloads and the two URLs in the inline
loader in `articles/globe.html`. Pass a new integer whenever the wasm changes
(e.g. `./deploy.sh 3`); the query is what keeps GitHub Pages’ 10-minute cache
from serving the old binary.

The page is already integrated: `articles/globe.html` has an inline
`<script type="module">` that imports `globe_app.js` and calls `init` with the
explicit `globe_app.wasm?v=` path. There is no `assets/js/globe.js` and no
cross-page prefetch anymore. The wasm creates the canvas inside
`#globe-container`, hides the skeleton (`globe-live`), and starts the slide-in
(`globe-ready` / `globe-settled`).

> The generated glue’s default wasm path is `globe-web_bg.wasm`; the inline
> loader passes the explicit `globe_app.wasm?v=` path instead.

> **First deploy only:** visitors holding the old page HTML in the 10-minute
> cache may need a hard reload; that HTML still points at the removed Bevy
> loader. After that, the `?v=` query handles future wasm updates.

The meta/OG copy and the page text now say “Rust and WebGL2”.

## How it works

- `src/math.rs` — small `Vec3` / column-major `Mat4` helpers; no glam.
- `src/land_data.rs` — the same baked `LAND_POSITIONS` array as the Bevy project.
- `src/renderer.rs` — shaders, buffers, VAOs, and the draw loop:
  - opaque navy sphere first (writes depth)
  - land dots as camera-facing point sprites, faded at the limb
  - city markers as larger soft sprites
  - route pulses as point sprites with per-vertex `t` / arc id; intensity is
    computed in the vertex shader from a `u_progress` uniform, so no buffer
    updates per frame
- `src/main.rs` — canvas creation, DPR-aware sizing, pointer drag, camera
  orbit, `requestAnimationFrame` loop, and the site’s class/timing contract.

The globe body uses the same near-black navy as the Bevy material plus a soft
key-light gradient. Land dots are neutral gray (the Bevy version’s dots render
around `#c8c8c8`).

## Notes

- The Bevy reference labelled `(18.8, 20.3)` as Paris, which is in Chad. This
  version uses the real coordinates (`48.8566, 2.3522`); the routes and
  markers are otherwise unchanged.
- DPR is capped at 2 to avoid rendering 3× buffers on phones.
- WebGL2 is required. If context creation or shader compilation fails, the
  wasm logs the error and replaces the skeleton with a short fallback message.
- The globe page still preloads its JS + wasm with `modulepreload` / `preload`;
  there is no cross-page prefetch, so other pages pay nothing for the globe.
- The globe page no longer loads `pdf.js`; it never used it. That removes a
  blocking third-party script from the critical path.
