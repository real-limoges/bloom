# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
# Type-check without producing WASM (fast iteration)
cargo check --target wasm32-unknown-unknown

# Build WASM module (output lands in pkg/)
wasm-pack build --target web

# Build with SIMD enabled (Tier 2 path)
RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack build --target web

# Native unit tests (no browser needed)
cargo test

# WASM integration tests (requires Chrome)
wasm-pack test --headless --chrome
```

> Prerequisites: `rustup target add wasm32-unknown-unknown` and `cargo install wasm-pack`

## Architecture

Bloom is a Rust→WASM graph visualization engine. Data flows through four independent layers:

```
Protocol → Graph → Layout → Render
```

Each layer has a clean interface to the next. The layout engine writes `(x, y)` into `Node` structs; the renderer reads them. Neither layer depends on the other's internals.

### Protocol (`src/protocol/`)
Decodes the BLOM binary format sent over WebSocket from Fugue (Elixir). Uses struct-of-arrays layout for cache-friendly decoding. `Decoder::decode_graph()` is the entry point; it returns a `Graph`.

BLOM wire format:
```
Header (16 bytes): magic u32, version u16, node_count u32, edge_count u32, flags u16
String Table (optional, HasLabels flag): total_len u32, offsets [u32; n], UTF-8 bytes
Node Data: ids [u32; n], pageranks [f32; n], degrees [u16; n]
Edge Data: sources [u32; n], targets [u32; n]
```
All integers are little-endian.

### Graph (`src/graph/`)
`Graph` stores nodes as `Vec<Node>` and edges as `Vec<Edge>` (edge list, not adjacency matrix). An `id_to_index: HashMap<u32, usize>` provides O(1) lookup from external database ID to array index. Node `(x, y)` fields start at `0.0` and are written by the layout engine each frame.

`spatial.rs` — quadtree for O(log n) mouse hit-testing. Imports `AABB` from `crate::spatial`.
`algorithms.rs` — PageRank (implemented), Louvain, shortest path, betweenness centrality (stubs).

### Shared Primitives (`src/spatial.rs`)
`AABB` (axis-aligned bounding box) lives here as a shared geometry primitive. Both the hit-testing quadtree (`graph/spatial.rs`) and the Barnes-Hut tree (`layout/barnes_hut.rs`) import it from this module. `AABB::enclosing()` computes a tight bounding box from an iterator of points; `padded()` expands it proportionally.

### Layout (`src/layout/`)
`ForceLayout::step()` runs one tick of the physics simulation: Barnes-Hut repulsion (O(n log n), θ=0.7), spring attraction (edges only), and gravity toward origin. Velocities are damped each step for convergence.

`barnes_hut.rs` implements the Barnes-Hut tree used by `force.rs` for O(n log n) repulsion. Uses `AABB` from `crate::spatial`.

### Render (`src/render/`)
GPU-accelerated rendering via `wgpu`.

`camera.rs` — `Camera` struct with exponential smoothing for pan/zoom, `world_to_screen`/`screen_to_world` coordinate transforms, `view_projection_matrix` for GPU uniform upload, and `focus_on` for snapping to a node.

`backend.rs` — `RenderBackend` wraps wgpu device/queue/surface. Async `new(canvas)` handles GPU initialization with WebGPU → WebGL2 fallback. Provides `begin_frame`/`end_frame` for the render loop.

`nodes.rs` — `NodeRenderer` uses instanced drawing: a unit quad with per-node instance data (`GpuNode`: position, size, color). Circle SDF fragment shader (`src/shaders/node.wgsl`) produces antialiased circles. Node size scales with PageRank.

`edges.rs` — `EdgeRenderer` draws edges as 1px lines (`PrimitiveTopology::LineList`). Each edge becomes two `GpuEdgeVertex` entries (source/target positions) built functionally via `filter_map`/`flatten`.

`text.rs` — `TextRenderer` renders node labels using SDF text. At init, rasterizes Inter font glyphs via `fontdue`, computes signed distance fields (Felzenszwalb EDT), and packs into a 1024x1024 `R8Unorm` atlas texture. Renders glyphs as instanced quads with adaptive `fwidth()`-based smoothing (`src/shaders/text.wgsl`). LOD culling skips labels when nodes are < 8px on screen.

Shaders are WGSL, included at compile time via `include_str!()` from `src/shaders/`.

### Entry Point (`src/lib.rs`)
`#[wasm_bindgen] BloomEngine` is the public JS API. It wraps `engine::BloomEngine` (the internal state machine). All internal errors use `Result<T, String>`; these are converted to `JsValue` only at the `#[wasm_bindgen]` boundary.

JS usage:
```js
const engine = new BloomEngine(canvas);
await engine.init_renderer(canvas);  // async GPU init
engine.load_graph(binaryData);
// animation loop:
engine.tick(dt);
```

`init_renderer` is async because `wgpu` adapter/device requests return Promises. The engine works without it (layout-only mode) — `tick()` skips rendering when no backend is present.

## Current Implementation State

| Module | Status |
|---|---|
| `protocol/format.rs` | Complete — BLOM header parsing |
| `protocol/decode.rs` | Complete — full decoder including string table, node/edge data, and all primitive readers |
| `protocol/mod.rs` | Complete — re-exports `Header`, `MAGIC`, `VERSION`, `Decoder` |
| `graph/types.rs` | Complete — `Node`, `Edge`, `Graph` |
| `spatial.rs` | Complete — shared `AABB` primitive (contains, intersects_circle, subdivide, enclosing, padded) |
| `graph/mod.rs` | Complete — declares `types`, `spatial`, `algorithms`; re-exports `Node`, `Edge`, `Graph`, `Quadtree`, `AABB` |
| `graph/spatial.rs` | Complete — `Quadtree` (insert, query_point, subdivide); imports `AABB` from `crate::spatial` |
| `graph/algorithms.rs` | Partial — `pagerank` implemented; `louvain`, `shortest_path`, `betweenness_centrality` are stubs |
| `layout/mod.rs` | Complete — re-exports `ForceLayout`, `ForceParams`, `BarnesHutTree` |
| `layout/force.rs` | Complete — `ForceParams` (with `theta`), `ForceLayout::new`/`step` with Barnes-Hut repulsion, attraction, gravity, damping |
| `layout/barnes_hut.rs` | Complete — `QuadNode` insert/subdivide, `compute_force` with θ approximation, `BarnesHutTree` wrapper |
| `render/mod.rs` | Complete — re-exports `backend`, `camera`, `nodes`, `edges`, `text` |
| `render/camera.rs` | Complete — `Camera` with smoothing, `focus_on`, `world_to_screen`, `screen_to_world`, `view_projection_matrix` |
| `render/backend.rs` | Complete — `RenderBackend` async GPU init (WebGPU/WebGL2), surface management, frame lifecycle |
| `render/nodes.rs` | Complete — `NodeRenderer` instanced quad rendering with `GpuNode`, pipeline, buffer management |
| `render/edges.rs` | Complete — `EdgeRenderer` line-list rendering with `GpuEdgeVertex`, pipeline, buffer management |
| `render/text.rs` | Complete — `TextRenderer` SDF text rendering with fontdue rasterization, Felzenszwalb EDT, atlas packing, instanced glyph quads, LOD culling; 5 tests |
| `shaders/node.wgsl` | Complete — circle SDF vertex/fragment shader with instancing and view-projection uniform |
| `shaders/edge.wgsl` | Complete — simple line vertex/fragment shader with view-projection uniform |
| `shaders/text.wgsl` | Complete — SDF text vertex/fragment shader with atlas sampling and `fwidth()` adaptive smoothing |
| `test_utils.rs` | Complete — `build_blom()` helper for constructing BLOM binary test data |
| `engine.rs` | Complete — owns `Graph`, `ForceLayout`, `Camera`, `Quadtree`, optional `RenderBackend`/`NodeRenderer`/`EdgeRenderer`/`TextRenderer`; `load_graph`, `tick` (layout + render), `resize`, `node_at`, `focus_node`, async `init_renderer`; 4 tests |
| `lib.rs` | Complete — `#[wasm_bindgen]` wrapper; exposes `load_graph`, `tick`, `resize`, `hover`, `focus_node`, async `init_renderer` to JS |

The implementation guide at `docs/IMPLEMENTATION_GUIDE.md` tracks the phased build plan. `docs/THEORY.md` explains the concepts behind each component.

## Project Context

Bloom is embedded in **Fugue** (Phoenix/Elixir LiveView app) as a git submodule at `assets/vendor/bloom/`. Fugue pushes binary BLOM data over WebSocket; Bloom renders it. The compiled `pkg/` directory is committed to this repo so Fugue can consume it without running wasm-pack locally.

Upstream data comes from **Dedalus** (separate Rust repo) which parses Wikipedia XML, computes PageRank, and writes to SQLite.
