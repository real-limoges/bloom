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

`spatial.rs` — quadtree for O(log n) mouse hit-testing.
`algorithms.rs` — PageRank, Louvain, shortest path, betweenness (stub, implement in Phase 5).

### Layout (`src/layout/`)
`ForceLayout::step()` runs one tick of the physics simulation: repulsion (all pairs, O(n²) initially), spring attraction (edges only), and gravity toward origin. Velocities are damped each step for convergence.

`barnes_hut.rs` replaces the O(n²) repulsion loop with an O(n log n) quadtree approximation in Phase 5.
`simd.rs` — WASM SIMD (`simd128`) paths for vectorized force calculations.

### Render (`src/render/`)
`backend.rs` detects the best available GPU tier at init (WebGPU → WebGL2+SIMD → WebGL2 → Canvas2D). The `wgpu` crate abstracts over WebGPU and WebGL2. Shaders are WGSL, included at compile time via `include_str!()` from `src/shaders/`.

Nodes are rendered with instanced drawing (one draw call for all nodes). Each node is a quad; the fragment shader applies a circle SDF for antialiased edges. Text uses an SDF font atlas (`assets/fonts/inter-sdf.png`).

### Entry Point (`src/lib.rs`)
`#[wasm_bindgen] BloomEngine` is the public JS API. It wraps `engine::BloomEngine` (the internal state machine). All internal errors use `Result<T, String>`; these are converted to `JsValue` only at the `#[wasm_bindgen]` boundary:

```rust
pub fn do_thing(&self) -> Result<(), String> { ... }

#[wasm_bindgen]
pub fn do_thing_js(&self) -> Result<(), JsValue> {
    self.do_thing().map_err(|e| JsValue::from_str(&e))
}
```

## Current Implementation State

| Module | Status |
|---|---|
| `protocol/format.rs` | Complete — BLOM header parsing |
| `protocol/decode.rs` | Complete — full decoder including string table, node/edge data, and all primitive readers |
| `protocol/mod.rs` | Complete — re-exports `Header`, `MAGIC`, `VERSION`, `Decoder` |
| `graph/types.rs` | Complete — `Node`, `Edge`, `Graph` |
| `graph/mod.rs` | Partial — only exports `types`; `spatial` and `algorithms` not yet wired in |
| `graph/spatial.rs` | Empty stub |
| `graph/algorithms.rs` | Empty stub |
| `layout/mod.rs` | Empty stub |
| `layout/force.rs` | Empty stub |
| `layout/barnes_hut.rs` | Empty stub |
| `layout/simd.rs` | Empty stub |
| `render/*` | All empty stubs |
| `engine.rs` | Empty stub |
| `lib.rs` | Minimal scaffold — `BloomEngine` has no fields yet |

The implementation guide at `docs/IMPLEMENTATION_GUIDE.md` tracks the phased build plan. `docs/THEORY.md` explains the concepts behind each component.

## Project Context

Bloom is embedded in **Fugue** (Phoenix/Elixir LiveView app) as a git submodule at `assets/vendor/bloom/`. Fugue pushes binary BLOM data over WebSocket; Bloom renders it. The compiled `pkg/` directory is committed to this repo so Fugue can consume it without running wasm-pack locally.

Upstream data comes from **Dedalus** (separate Rust repo) which parses Wikipedia XML, computes PageRank, and writes to SQLite.
