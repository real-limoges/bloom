# Bloom — Development Guidelines

## Overview

Bloom is a Rust→WASM graph visualization engine. It compiles to WebAssembly with `wasm-pack` and renders graphs using `wgpu` (WebGPU/WebGL2).

## Build Commands

```bash
# Standard build
wasm-pack build --target web

# Build with SIMD
RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack build --target web

# Run tests
cargo test
wasm-pack test --headless --chrome

# Check without building WASM (faster iteration)
cargo check --target wasm32-unknown-unknown
```

## Code Organization

```
src/
├── lib.rs          # wasm-bindgen entry, BloomEngine exports
├── engine.rs       # Engine state machine, lifecycle
├── render/         # GPU rendering (wgpu)
├── layout/         # Force-directed layout (Barnes-Hut)
├── graph/          # Graph data structures, algorithms
├── protocol/       # BLOM binary format decoder
└── shaders/        # WGSL shader source files
```

## Key Architectural Rules

### Separation of Concerns

The four layers (protocol, graph, layout, render) are strictly separated:
- Protocol decodes bytes into Graph types. It does not know about rendering.
- Graph stores data and runs algorithms. It does not know about layout positions.
- Layout reads graph topology, writes position floats. It does not know about GPU.
- Render reads positions and graph metadata, draws to screen. It does not know about layout algorithms.

This separation is what makes the tiered fallback work — you can swap the compute backend without touching the renderer, and vice versa.

### Position Buffer

The layout engine writes to a `Vec<f32>` (interleaved x,y pairs). The renderer reads this same buffer. This is the contract between them. No other data format flows between layout and render.

```rust
// Layout writes:
positions[i * 2]     = x;
positions[i * 2 + 1] = y;

// Renderer reads:
let x = positions[i * 2];
let y = positions[i * 2 + 1];
```

### wasm-bindgen Exports

All public API functions go through `BloomEngine` in `lib.rs`. Do not export internal types or functions via `#[wasm_bindgen]`. The JS-facing API should be minimal and stable.

### No `unwrap()` in Production Paths

All error handling in code that runs during normal operation must use `Result`. `unwrap()` is only acceptable in:
- Tests
- Static initialization that cannot fail
- Code paths that would indicate a programming error (document with comments)

Use `JsValue` as the error type for wasm-bindgen exports.

### GPU Resource Management

- Create GPU resources (buffers, textures, pipelines) during initialization, not per-frame
- Resize buffers when graph size changes, not every frame
- Use `bytemuck` for zero-copy buffer uploads
- Always clean up in `destroy()`

### SIMD

Use `cfg` attributes for SIMD paths:

```rust
#[cfg(target_feature = "simd128")]
fn force_simd(positions: &mut [f32], forces: &[f32]) {
    // WASM SIMD implementation
}

#[cfg(not(target_feature = "simd128"))]
fn force_simd(positions: &mut [f32], forces: &[f32]) {
    // Scalar fallback
}
```

Build with SIMD: `RUSTFLAGS="-Ctarget-feature=+simd128" wasm-pack build --target web`

### Shaders

WGSL shaders are in `src/shaders/` and included at compile time:

```rust
let shader_source = include_str!("shaders/node.wgsl");
```

Do not generate shader code at runtime. All shaders are static.

## Performance Guidelines

- Target: 20K nodes at >30fps after layout settles
- Use instanced rendering (one draw call per primitive type, not per node)
- Barnes-Hut approximation (theta = 0.7) for O(n log n) force calculation
- Quadtree for O(log n) spatial queries (hover hit-testing)
- Avoid allocations in the render loop — pre-allocate buffers
- Profile with `web_sys::Performance::now()` and the browser profiler

## Testing Strategy

- **Unit tests**: Graph algorithms, binary protocol decoding, quadtree operations
- **WASM integration tests**: `wasm-bindgen-test` for browser-specific behavior
- **Visual tests**: `examples/standalone.html` for manual rendering verification
- **Benchmarks**: Layout convergence time at various node counts

## Dependencies Policy

Keep dependencies minimal. Every dependency increases WASM binary size.

Required:
- `wasm-bindgen` — JS interop
- `wgpu` — GPU abstraction
- `web-sys` / `js-sys` — browser APIs
- `bytemuck` — zero-copy GPU buffer casting
- `glam` — math types (vec2, mat4)
- `log` + `wasm-logger` — logging

Avoid:
- Serialization frameworks (serde, bincode) — we have a custom binary protocol
- Async runtimes — use callbacks and requestAnimationFrame
- Heavy math libraries — glam covers what we need
