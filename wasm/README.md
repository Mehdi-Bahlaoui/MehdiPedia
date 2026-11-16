# `wasm/` — `mini3d` Engine & WebGL2 Scenes

Rust + WebGL2 workspace for the site's 3D WebAssembly animations — no Bevy, no
runtime asset fetches, and tiny bundles.

Build source lives here. Each scene's page, page CSS, and compiled bundle live
together under `articles/<scene>/`:

```text
wasm/
├── Cargo.toml                  # workspace + shared release profile (opt-level="z", LTO)
├── deploy.sh                   # ./deploy.sh <scene> [cache-version]
├── crates/
│   └── mini3d/                 # reusable 3D WebGL2 micro-engine (ready to cut to its own repo)
│       └── src/
│           ├── lib.rs
│           ├── app.rs          # <canvas> creation, DPR sizing, RAF loop, slide-in lifecycle
│           ├── camera.rs       # OrbitCamera + pointer-drag controls
│           ├── geo.rs          # lat/lon, slerp, great-circle arcs
│           ├── gfx.rs          # shaders, programs, buffers, VAOs, RenderState, geometry builders
│           ├── material.rs     # SolidMaterial, PointMaterial, PulseMaterial, QuadMaterial
│           ├── math.rs         # Vec3, Quat, column-major Mat4
│           ├── render.rs       # Mesh, Points, PulseLine, InstancedQuads, Renderer
│           └── scene.rs        # Scene trait
└── scenes/
    ├── globe/                  # Interactive dot globe source
    │   └── src/
    │       ├── main.rs         # ~25 lines: App + OrbitConfig + Lifecycle
    │       ├── globe.rs        # ~125 lines: Globe scene + materials
    │       └── land_data.rs    # 9,251 baked land positions
    └── confetti/               # Falling confetti source (InstancedQuads + Quat)
        └── src/
            └── main.rs         # ~245 lines: finite 1,200-piece launcher burst, 3D depth
```

Site output:

```text
articles/
├── globe/
│   ├── index.html              # /articles/globe/
│   ├── globe.css
│   └── wasm/
│       ├── globe_app.js
│       └── globe_app.wasm
└── confetti/
    ├── index.html              # /articles/confetti/ ("The confetti is amazing")
    ├── confetti.css
    └── wasm/
        ├── confetti_app.js
        └── confetti_app.wasm
```

## Bundle sizes

| Scene | Raw WASM | Gzipped WASM | Gzipped JS | Notes |
|---|---:|---:|---:|---|
| `globe` | ~157 KB | **~121 KB** | ~5 KB | Includes 111 KB of baked land coordinates |
| `confetti` | ~42 KB | **~20 KB** | ~5 KB | Finite 1,200-piece launcher burst in 1 draw call; 3D depth, vivid/emissive paper, density scales down on narrow screens |

Each scene is its own binary crate. With `lto = true` and per-primitive shader
ownership, unused engine primitives are stripped from each `.wasm`.

## Build & Dev

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk        # if you don't have it

# Check the whole workspace
cargo check --workspace --target wasm32-unknown-unknown

# Run a scene locally with Trunk
cd scenes/globe && trunk serve       # http://127.0.0.1:8080
cd scenes/confetti && trunk serve    # http://127.0.0.1:8081
```

## Deploy into the site

```bash
./deploy.sh globe 3
```

`./deploy.sh` builds `scenes/<scene>` in release mode, copies:

- `scenes/<scene>/dist/<scene>.js` → `articles/<scene>/wasm/<scene>_app.js`
- `scenes/<scene>/dist/<scene>_bg.wasm` → `articles/<scene>/wasm/<scene>_app.wasm`

and, for mapped pages (`globe` → `articles/globe/index.html`, `confetti` →
`articles/confetti/index.html`), bumps the `?v=` query on the
`<link rel="modulepreload">` / `<link rel="preload">` tags and the inline
loader. Pass a new integer whenever the wasm changes so GitHub Pages' 10-minute
cache serves the fresh binary.

## Adding a new scene

1. Copy `scenes/confetti` (or `scenes/globe`) to `scenes/<name>` and add
   `"scenes/<name>"` to `members` in `wasm/Cargo.toml`.
2. Implement `mini3d::scene::Scene` (`update` + `render`) and launch with
   `mini3d::app::App::new("<container-id>", "<aria-label>").run(...)`.
3. Create `articles/<name>/index.html` and run `./deploy.sh <name>` to emit
   `articles/<name>/wasm/<name>_app.{js,wasm}`.

## Cutting `mini3d` into its own repository / crates.io

`crates/mini3d` has zero scene-specific dependencies. When moving it to a
standalone repository:

1. Copy `crates/mini3d/` to the new repo.
2. Replace `wasm-bindgen.workspace = true` and `web-sys.workspace = true` in
   `mini3d/Cargo.toml` with normal semver dependencies (`wasm-bindgen = "0.2.108"`,
   `web-sys = { version = "0.3.85", features = [...] }`).
3. In `wasm/Cargo.toml`, switch `mini3d = { path = "crates/mini3d" }` to the
   published version (`mini3d = "0.1"`).
