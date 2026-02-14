# Bloom

A Rust→WASM graph visualization engine. Renders knowledge graphs with GPU-accelerated WebGL/WebGPU, force-directed layout, and a binary data protocol. Built to be embedded in [Fugue](https://github.com/real-limoges/fugue) and fed by [Dedalus](https://github.com/real-limoges/dedalus).

## What This Is

Bloom is a browser-based graph renderer compiled from Rust to WebAssembly. It takes packed binary graph data, computes a force-directed layout, and renders the result using `wgpu` (which targets WebGPU or WebGL2 depending on browser support). The host application (Fugue, a Phoenix LiveView app) handles data fetching, search, and UI chrome — Bloom handles everything visual.

The pipeline: Wikipedia XML → **Dedalus** (Rust extraction) → SQLite → **Fugue** (Elixir serving) → binary WebSocket → **Bloom** (WASM rendering)

## Architecture

```
┌─────────────────────────────────────────────┐
│  BloomEngine (wasm-bindgen entry point)      │
│                                              │
│  ┌──────────────────────────────────────┐    │
│  │ Protocol Layer                        │    │
│  │  decode.rs — BLOM binary format      │    │
│  │  format.rs — magic bytes, versioning │    │
│  └──────────┬───────────────────────────┘    │
│             ↓                                │
│  ┌──────────────────────────────────────┐    │
│  │ Graph Layer                           │    │
│  │  types.rs    — Node, Edge, Graph     │    │
│  │  spatial.rs  — Quadtree (hit-test)   │    │
│  │  algorithms.rs — Louvain, PageRank,  │    │
│  │                  shortest path       │    │
│  └──────────┬───────────────────────────┘    │
│             ↓                                │
│  ┌──────────────────────────────────────┐    │
│  │ Layout Layer                          │    │
│  │  barnes_hut.rs — N-body simulation   │    │
│  │  force.rs      — spring/repulsion    │    │
│  │  simd.rs       — WASM SIMD paths     │    │
│  │  Output: Vec<f32> position buffer    │    │
│  └──────────┬───────────────────────────┘    │
│             ↓                                │
│  ┌──────────────────────────────────────┐    │
│  │ Render Layer                          │    │
│  │  backend.rs — WebGPU/WebGL2 detect   │    │
│  │  nodes.rs   — Instanced circles      │    │
│  │  edges.rs   — Line segments/beziers  │    │
│  │  text.rs    — SDF font rendering     │    │
│  │  camera.rs  — Pan, zoom, transitions │    │
│  └──────────────────────────────────────┘    │
└──────────────────────────────────────────────┘
```

## GPU Fallback Tiers

Bloom detects browser capabilities at init and selects the best backend:

| Tier | Render | Compute | Availability | Notes |
|------|--------|---------|-------------|-------|
| 1 | WebGPU | GPU compute shaders | Chrome 113+, Edge 113+ | Showcase tier. Layout runs on GPU. |
| 2 | WebGL2 | WASM SIMD (128-bit) | ~95% browsers | Default path. Barnes-Hut vectorizes well. |
| 3 | WebGL2 | WASM scalar | ~97% browsers | Still faster than JS layout libraries. |
| 4 | Canvas 2D | WASM scalar | Everything | Emergency fallback. Functional, not pretty. |

The layout engine writes to a flat `Vec<f32>` position buffer. The renderer reads it. Neither cares which backend the other uses.

## Binary Protocol (BLOM)

Bloom receives graph data as a compact binary format, not JSON. Struct-of-arrays layout for SIMD-friendly decoding:

```
Header (16 bytes)
  magic:      u32  = 0x424C4F4D ("BLOM")
  version:    u16
  node_count: u32
  edge_count: u32
  flags:      u16

String Table
  total_len:  u32
  offsets:    [u32; node_count]
  data:       UTF-8 bytes

Node Data (packed arrays)
  ids:        [u32; node_count]
  pageranks:  [f32; node_count]
  degrees:    [u16; node_count]

Edge Data
  sources:    [u32; edge_count]
  targets:    [u32; edge_count]
```

Encoded by Fugue (Elixir), decoded by Bloom (Rust). Zero JSON parsing in the hot path.

## Project Structure

```
bloom/
├── Cargo.toml
├── rust-toolchain.toml
├── src/
│   ├── lib.rs                    wasm-bindgen entry, BloomEngine export
│   ├── engine.rs                 Engine lifecycle, state machine
│   ├── render/
│   │   ├── mod.rs
│   │   ├── backend.rs            Capability detection, backend selection
│   │   ├── nodes.rs              Instanced circle rendering
│   │   ├── edges.rs              Edge rendering (lines, beziers)
│   │   ├── text.rs               SDF text rendering
│   │   └── camera.rs             Pan, zoom, animated transitions
│   ├── layout/
│   │   ├── mod.rs
│   │   ├── barnes_hut.rs         Quadtree + N-body force simulation
│   │   ├── force.rs              Spring attraction, repulsion, gravity
│   │   └── simd.rs               WASM SIMD specializations
│   ├── graph/
│   │   ├── mod.rs
│   │   ├── types.rs              Node, Edge, Graph structs
│   │   ├── algorithms.rs         Louvain, PageRank, shortest path, betweenness
│   │   └── spatial.rs            Quadtree for spatial queries (hover, click)
│   ├── protocol/
│   │   ├── mod.rs
│   │   ├── decode.rs             BLOM binary decoder
│   │   └── format.rs             Constants, magic bytes, version
│   └── shaders/
│       ├── node.wgsl             Instanced vertex + fragment
│       ├── edge.wgsl             Edge vertex + fragment
│       └── text.wgsl             SDF text fragment
├── assets/
│   └── fonts/
│       └── inter-sdf.png         Pre-built SDF font atlas
├── pkg/                           wasm-pack output (committed to repo)
│   ├── bloom_bg.wasm
│   ├── bloom.js
│   ├── bloom.d.ts
│   └── package.json
├── tests/
│   ├── decode_test.rs
│   ├── layout_test.rs
│   └── spatial_test.rs
├── examples/
│   └── standalone.html            Self-contained demo (no Fugue needed)
└── README.md                      This file
```

## Build

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build for browser (ES module output)
wasm-pack build --target web

# Build with SIMD enabled (requires nightly or -Ctarget-feature=+simd128)
RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack build --target web

# Output lands in pkg/
ls pkg/
# bloom_bg.wasm  bloom.js  bloom.d.ts  package.json
```

## Cargo.toml

```toml
[package]
name = "bloom"
version = "0.1.0"
edition = "2021"
description = "GPU-accelerated graph visualization engine for the browser"
license = "BSD-3-Clause"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
wgpu = { version = "24", features = ["webgl"] }
js-sys = "0.3"
web-sys = { version = "0.3", features = [
  "Window",
  "Document",
  "HtmlCanvasElement",
  "WebGl2RenderingContext",
  "GpuDevice",
  "GpuAdapter",
  "Navigator",
  "Gpu",
  "MouseEvent",
  "WheelEvent",
  "KeyboardEvent",
  "ResizeObserver",
  "ResizeObserverEntry",
  "DomRectReadOnly",
  "Performance",
] }
log = "0.4"
wasm-logger = "0.2"
bytemuck = { version = "1", features = ["derive"] }       # zero-copy GPU buffer casting
glam = { version = "0.29", features = ["bytemuck"] }      # math (vec2, mat4)

[dev-dependencies]
wasm-bindgen-test = "0.3"

[profile.release]
opt-level = "z"       # optimize for size (WASM download matters)
lto = true
codegen-units = 1
strip = true
```

## JS API (wasm-bindgen exports)

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct BloomEngine { /* ... */ }

#[wasm_bindgen]
impl BloomEngine {
    /// Create engine, attach to a canvas element, detect GPU backend
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: web_sys::HtmlCanvasElement) -> Result<BloomEngine, JsValue>;

    /// Load graph from BLOM binary format
    pub fn load_graph(&mut self, data: &[u8]) -> Result<(), JsValue>;

    /// Run N iterations of force-directed layout
    pub fn step_layout(&mut self, iterations: u32);

    /// Render current state to canvas
    pub fn render(&mut self);

    /// Start animation loop (layout + render at requestAnimationFrame rate)
    pub fn start(&mut self);

    /// Stop animation loop
    pub fn stop(&mut self);

    /// Highlight specific nodes (by ID)
    pub fn highlight_nodes(&mut self, ids: &[u32]);

    /// Clear highlights
    pub fn clear_highlights(&mut self);

    /// Focus camera on a node (smooth animation)
    pub fn focus_node(&mut self, id: u32);

    /// Fit all nodes in view
    pub fn fit_view(&mut self);

    /// Register callback: node clicked
    pub fn on_node_click(&mut self, callback: &js_sys::Function);

    /// Register callback: node hovered
    pub fn on_node_hover(&mut self, callback: &js_sys::Function);

    /// Get current backend tier (1-4)
    pub fn backend_tier(&self) -> u8;

    /// Get current FPS
    pub fn fps(&self) -> f32;

    /// Clean up GPU resources
    pub fn destroy(&mut self);
}
```

## Integration with Fugue

Bloom is consumed by Fugue as a git submodule:

```bash
# In the fugue repo
git submodule add https://github.com/real-limoges/bloom.git assets/vendor/bloom

# Or, submodule just the pkg/ directory (if you use sparse checkout)
```

The LiveView hook imports from the submodule:

```javascript
// fugue/assets/js/hooks/bloom_hook.js
import init, { BloomEngine } from '../vendor/bloom/pkg/bloom.js';
```

LiveView pushes binary data, Bloom renders it. LiveView handles search, article details, and UI chrome. Bloom handles everything visual.

## Development Workflow

```
1. Edit Rust in bloom/src/
2. wasm-pack build --target web
3. In fugue/: git submodule update (or symlink pkg/ during dev)
4. Phoenix picks up changes via esbuild
5. Refresh browser
```

For faster iteration during development, symlink `bloom/pkg/` into `fugue/assets/vendor/bloom/` instead of using the submodule. Switch back to submodule for releases.

## Standalone Demo

`examples/standalone.html` loads Bloom without Fugue — useful for development and debugging:

```html
<!DOCTYPE html>
<html>
<head><title>Bloom Standalone</title></head>
<body>
  <canvas id="graph" width="1200" height="800"></canvas>
  <script type="module">
    import init, { BloomEngine } from '../pkg/bloom.js';
    await init();
    const canvas = document.getElementById('graph');
    const engine = new BloomEngine(canvas);

    // Load test data (generate random graph)
    const response = await fetch('test_graph.blom');
    const data = new Uint8Array(await response.arrayBuffer());
    engine.load_graph(data);
    engine.start();
  </script>
</body>
</html>
```

## Shaders

WGSL shaders live in `src/shaders/` and are included via `include_str!()` at compile time.

### Node shader (instanced)

Each node is a unit quad, scaled and positioned per-instance:

```wgsl
// node.wgsl
struct Camera {
  view_proj: mat4x4<f32>,
  viewport: vec2<f32>,
}

struct NodeInstance {
  @location(0) position: vec2<f32>,  // from layout engine
  @location(1) size: f32,            // from pagerank
  @location(2) color: vec4<f32>,     // from community/degree
}

@group(0) @binding(0) var<uniform> camera: Camera;

@vertex
fn vs_main(
  @builtin(vertex_index) vi: u32,
  instance: NodeInstance,
) -> @builtin(position) vec4<f32> {
  // Unit quad vertices
  let quad = array<vec2<f32>, 4>(
    vec2(-1.0, -1.0), vec2(1.0, -1.0),
    vec2(-1.0,  1.0), vec2(1.0,  1.0),
  );
  let local = quad[vi] * instance.size;
  let world = vec4(instance.position + local, 0.0, 1.0);
  return camera.view_proj * world;
}

@fragment
fn fs_main(
  @builtin(position) pos: vec4<f32>,
  @location(0) color: vec4<f32>,
  @location(1) uv: vec2<f32>,
) -> @location(0) vec4<f32> {
  // Circle SDF
  let d = length(uv);
  let alpha = 1.0 - smoothstep(0.9, 1.0, d);
  return vec4(color.rgb, color.a * alpha);
}
```

## Testing

```bash
# Rust unit tests
cargo test

# WASM integration tests (requires browser or wasm-pack test)
wasm-pack test --headless --chrome

# Layout benchmark
cargo bench --bench layout_bench
```

## Related

- [Dedalus](https://github.com/real-limoges/dedalus) — Rust pipeline that produces the graph data
- [Fugue](https://github.com/real-limoges/fugue) — Phoenix/Elixir app that serves data and hosts Bloom
- [wgpu](https://docs.rs/wgpu) — GPU abstraction layer (WebGPU + WebGL2)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) — Rust→WASM build toolchain
