# Bloom — Theory & Motivation

This document explains the *why* behind every significant technical decision in Bloom. The [Implementation Guide](./IMPLEMENTATION_GUIDE.md) tells you what to write; this document explains what the code is actually doing and why it works.

---

## Table of Contents

1. [Why Rust + WebAssembly?](#1-why-rust--webassembly)
2. [The Browser Execution Model](#2-the-browser-execution-model)
3. [Binary Protocol (BLOM)](#3-binary-protocol-blom)
4. [Graph Data Structures](#4-graph-data-structures)
5. [Force-Directed Layout](#5-force-directed-layout)
6. [Barnes-Hut: Taming the N-Body Problem](#6-barnes-hut-taming-the-n-body-problem)
7. [WASM SIMD: Parallelism Without a GPU](#7-wasm-simd-parallelism-without-a-gpu)
8. [GPU Rendering Pipeline](#8-gpu-rendering-pipeline)
9. [Instanced Rendering](#9-instanced-rendering)
10. [Signed Distance Field (SDF) Text](#10-signed-distance-field-sdf-text)
11. [Spatial Indexing with a Quadtree](#11-spatial-indexing-with-a-quadtree)
12. [Camera & Coordinate Systems](#12-camera--coordinate-systems)
13. [The Full Pipeline](#13-the-full-pipeline)

---

## 1. Why Rust + WebAssembly?

### The problem with JavaScript for graph rendering

A knowledge graph with 10,000 nodes requires roughly 50 million repulsion calculations per layout step (every node pushing away every other node). JavaScript is a dynamically-typed, garbage-collected language. Those properties are great for UI logic, but they are a disaster for tight numerical loops because:

- **The garbage collector pauses the entire thread** at unpredictable moments. A 10ms GC pause mid-frame means you miss your 60 FPS deadline.
- **JavaScript numbers are 64-bit floats** — every arithmetic operation boxes and unboxes values, causing memory allocation that feeds the GC.
- **The JS JIT compiler** can produce fast code, but it requires type stability. Mixed types, polymorphism, and megamorphic call sites de-optimize hot paths.

Existing JS graph libraries (d3-force, etc.) solve this by limiting graph size or accepting lower frame rates. We want both: large graphs *and* smooth animation.

### What WebAssembly is

WebAssembly (WASM) is a binary instruction format that browsers can execute. Think of it as a portable, safe assembly language that runs in the browser's sandboxed virtual machine. It is *not* JavaScript and does not share JavaScript's garbage collection model.

Key properties:
- **Linear memory**: WASM has a flat array of bytes it controls. No GC, no object overhead, no surprise pauses.
- **Predictable performance**: The browser JIT compiles WASM to machine code with much less dynamic speculation than JS. What you write is much closer to what runs.
- **Deterministic sizing**: A `f32` in WASM is always 4 bytes. An array of 10,000 `f32` values is always 40,000 bytes. You can reason about cache behavior.

### Why Rust specifically

Any language that compiles to WASM would give you the above. Rust was chosen because:

- **No runtime, no GC**: Rust has zero garbage collection. Memory is managed at compile time through the ownership system. This means the WASM binary has no GC runtime embedded in it, keeping the `.wasm` download small.
- **`bytemuck` and zero-copy**: Rust lets you cast a `Vec<GpuNode>` directly to `&[u8]` for GPU upload without any copying. In JS this requires typed arrays and manual byte offsets.
- **`glam`'s SIMD math**: The `glam` crate uses SIMD intrinsics for 2D/3D vector math automatically. You write `pos_i - pos_j` and get vectorized subtraction.
- **Safety without overhead**: The Rust compiler catches memory bugs, null pointer dereferences, and data races at compile time. You get C-like performance without C-like crashes.

---

## 2. The Browser Execution Model

Before going further, it helps to understand how code runs in a browser.

### The main thread

Browsers are single-threaded by default. JavaScript, DOM manipulation, layout, and paint all share one thread. If anything blocks that thread for more than ~16ms, the browser drops a frame. A 100ms block makes the page feel frozen.

### The event loop

The browser runs a continuous loop: process events → run JS → paint → repeat. Your animation loop plugs into this via `requestAnimationFrame`, which calls your code once per paint cycle. You have a budget of ~16ms to do layout math and submit draw calls.

### WASM and the main thread

WASM runs *on* the main thread by default — the same thread as JavaScript. But because WASM has no GC and Rust produces tight machine code, the same 16ms budget goes much further than with JS.

### `wasm-bindgen`: the glue layer

Raw WASM can only pass numbers between JS and WASM. `wasm-bindgen` is a tool that generates the bridge code so you can write:

```rust
#[wasm_bindgen]
pub fn load_graph(&mut self, data: &[u8]) -> Result<(), JsValue>
```

...and call it from JavaScript as:

```js
engine.load_graph(new Uint8Array(binaryData));
```

The `#[wasm_bindgen]` attribute generates JavaScript wrapper code that handles pointer passing, memory lifetime, and type conversion. You write ergonomic Rust; users get ergonomic JS.

---

## 3. Binary Protocol (BLOM)

### Why not JSON?

The naive approach to sending graph data is JSON:

```json
{
  "nodes": [{"id": 1, "label": "Alice", "pagerank": 0.5}, ...],
  "edges": [{"source": 1, "target": 2}, ...]
}
```

For a 10,000-node graph this produces roughly **1–3 MB of text**. Then:

1. The browser must allocate a string to hold it.
2. A JSON parser must walk every character and allocate JS objects.
3. Bloom must convert each JS object back into a Rust struct.

This is three allocations and two copies per node. For 10,000 nodes that is 30,000 allocations just to start.

### Struct-of-arrays layout

BLOM uses a *struct-of-arrays* layout instead of *array-of-structs*. The difference:

**Array of structs** (JSON-like):
```
[{id: 0, pagerank: 0.5, degree: 3}, {id: 1, pagerank: 0.3, degree: 2}, ...]
```
Each node's fields are interleaved. To read all pageranks, you hop across memory with stride equal to the struct size.

**Struct of arrays** (BLOM):
```
ids:       [0, 1, 2, ...]         ← all IDs together
pageranks: [0.5, 0.3, 0.8, ...]   ← all pageranks together
degrees:   [3, 2, 5, ...]         ← all degrees together
```
All values of the same type are contiguous. To read all pageranks: one sequential memory read. This is *cache-friendly* because the CPU prefetches sequential memory automatically.

### Why a magic number?

The first 4 bytes of every BLOM message are `0x424C4F4D` — the ASCII encoding of "BLOM". This is a *magic number*: a sentinel that lets the decoder verify it received the right kind of data before doing anything. Without it, a corrupted WebSocket frame or wrong HTTP response would be parsed as graph data, producing garbage or a crash.

### Little-endian encoding

All integers are stored in *little-endian* byte order (least significant byte first). x86 and ARM processors are natively little-endian, so reading a `u32` from little-endian bytes requires no byte swapping — the CPU does it "for free."

---

## 4. Graph Data Structures

### What a graph actually is

A graph is a set of *nodes* (also called vertices) and *edges* (connections between nodes). In Bloom's domain, nodes are Wikipedia articles and edges are hyperlinks between them.

Two fundamental representations:

**Adjacency matrix**: A 2D boolean array where `matrix[i][j] = true` means there's an edge from node `i` to node `j`. Fast to query but uses O(n²) memory — 10,000 nodes would need 100 million bits (12 MB).

**Edge list**: A flat list of `(source, target)` pairs. Uses O(e) memory where `e` is edge count. Bloom uses this because Wikipedia graphs are *sparse* — most nodes connect to a small fraction of others.

### The `id_to_index` HashMap

Nodes have externally-assigned IDs (from the database) that may not be contiguous or zero-indexed. The `id_to_index: HashMap<u32, usize>` maps from external ID to the array index where that node lives. This makes `graph.node_by_id(id)` an O(1) operation instead of an O(n) scan.

### PageRank

PageRank is an algorithm invented for Google Search. It assigns each node a score based on the number and quality of incoming links. Intuition: if many important articles link to you, you are probably important.

The formula is iterative:
```
PageRank(node) = (1 - d) + d * Σ(PageRank(linking_node) / out_degree(linking_node))
```
where `d ≈ 0.85` is a damping factor. You repeat until values converge.

Bloom uses PageRank scores (pre-computed by Dedalus) to size nodes — higher PageRank nodes appear larger. This makes the most important concepts visually prominent.

---

## 5. Force-Directed Layout

### The core metaphor

Force-directed layout simulates a physical system: edges behave like springs that pull connected nodes together, and nodes behave like charged particles that repel each other. You run the simulation until the system reaches equilibrium (low energy), then render.

Why does this produce useful layouts? Because:
- **Springs** encode relationship. Connected nodes want to be near each other.
- **Repulsion** prevents overlapping. Unrelated nodes spread apart.
- **Gravity** keeps the graph centered and prevents it from drifting off-screen.

The equilibrium position reveals the graph's structure — clusters of highly-connected nodes appear as visual communities.

### Fruchterman-Reingold forces

Bloom's basic layout uses the Fruchterman-Reingold model (1991):

**Repulsion**: Every pair of nodes pushes each other away. Force is inversely proportional to distance squared — like electrostatic repulsion:
```
F_repulsion = repulsion_constant / distance²
```
Applied along the vector connecting the two nodes (the direction they push apart).

**Attraction**: Only connected nodes pull together. Force is proportional to distance — like a spring:
```
F_attraction = attraction_constant * distance
```

**Gravity**: Every node is weakly pulled toward the origin `(0, 0)`:
```
F_gravity = -gravity_constant * position
```
Without this, the entire graph drifts in random directions.

**Damping**: Velocities are multiplied by a factor slightly less than 1.0 each step. This acts like friction, slowing the simulation so it converges instead of oscillating forever.

### Time stepping

Each call to `step()` advances the simulation by one discrete time step:
1. Compute all forces on each node.
2. Update velocity: `velocity = (velocity + force) * damping`
3. Update position: `position += velocity`

This is *Euler integration* — the simplest numerical integrator. More sophisticated methods (Runge-Kutta) are more stable but overkill here.

### O(n²) problem

The naive repulsion algorithm computes all pairs: for 10,000 nodes that's 50 million calculations per frame. This is why Phase 3 starts O(n²) and Phase 5 replaces it with Barnes-Hut.

---

## 6. Barnes-Hut: Taming the N-Body Problem

### The N-body problem

Calculating the gravitational (or repulsive) force between every pair of N bodies is O(n²). This is the *N-body problem* in physics. For Bloom with 10K nodes, 50M calculations per 16ms frame is simply not feasible.

### The key insight: distant clusters

If a group of nodes is far away from the node you're computing forces for, you can treat the *entire group* as a single body at its center of mass. The approximation error is small when the group is distant relative to its size.

This is the Barnes-Hut approximation (1986), originally developed for astronomical N-body simulations.

### How the quadtree enables this

A *quadtree* partitions 2D space by recursively dividing each region into four quadrants until each region contains at most one node.

```
┌───────┬───────┐
│   A   │ C │ D │
│       ├───┴───┤
│       │   E   │
├───────┼───────┤
│   B   │   F   │
│       │       │
└───────┴───────┘
```

Each internal node of the tree stores the *center of mass* and *total mass* of all nodes in that region.

When computing the force on node `X`:
1. Start at the root of the quadtree.
2. For each region, compute `s/d` where `s` is the region's side length and `d` is the distance from `X` to the region's center of mass.
3. If `s/d < θ` (some threshold, typically 0.5–1.0), treat the whole region as a single body.
4. Otherwise, recurse into the region's children.

The parameter θ controls the accuracy/speed tradeoff: smaller θ means more accurate (more recursion) but slower; larger θ is faster but less accurate.

This reduces complexity from O(n²) to **O(n log n)** — a massive improvement for large graphs.

---

## 7. WASM SIMD: Parallelism Without a GPU

### What SIMD is

SIMD stands for *Single Instruction, Multiple Data*. Modern CPUs can execute one instruction that operates on multiple values simultaneously.

For example, adding two arrays of 4 floats:

**Scalar (no SIMD)**:
```
result[0] = a[0] + b[0]   // 1 instruction
result[1] = a[1] + b[1]   // 1 instruction
result[2] = a[2] + b[2]   // 1 instruction
result[3] = a[3] + b[3]   // 1 instruction
// 4 instructions total
```

**SIMD (128-bit)**:
```
result[0..4] = a[0..4] + b[0..4]  // 1 instruction for all 4
```

A 128-bit SIMD register holds four 32-bit floats simultaneously. WebAssembly's `simd128` feature exposes these instructions.

### Why this matters for layout

The repulsion loop processes thousands of `(x, y)` position pairs. With SIMD, you can process 4 nodes at a time using vectorized arithmetic. The struct-of-arrays layout in BLOM is designed specifically to enable this: contiguous arrays of `x` coordinates and `y` coordinates can be loaded directly into SIMD registers.

### WASM SIMD availability

WASM SIMD (the `simd128` target feature) is supported in ~95% of modern browsers and is the reason Tier 2 is the default path — you get 4x theoretical speedup on the layout math with no GPU required.

---

## 8. GPU Rendering Pipeline

### Why not just draw with Canvas 2D?

The Canvas 2D API (`ctx.arc()`, `ctx.lineTo()`) is simple but every draw call goes through the browser's 2D compositor, which is single-threaded CPU work. Drawing 10,000 circles means 10,000 separate draw calls. At a certain scale this becomes the bottleneck.

### The GPU pipeline

A GPU is a massively parallel processor designed to execute the same code on thousands of inputs simultaneously. A modern GPU has thousands of shader cores that can all execute in parallel. Drawing 10,000 nodes can become a single draw call that the GPU handles in parallel.

The rendering pipeline in Bloom uses the GPU like this:

1. **Vertex shader**: Runs once per vertex on the GPU. Transforms node positions from world space to screen space.
2. **Rasterization**: The GPU hardware converts triangles into pixels (fragments).
3. **Fragment shader**: Runs once per pixel on the GPU. Determines the final color.

Both shaders run on thousands of cores simultaneously.

### WebGPU vs WebGL2

**WebGL2** is the established GPU API for browsers, available since ~2017. It's a web wrapper around OpenGL ES 3.0. WebGL2 is available on ~97% of browsers.

**WebGPU** is the next-generation API, closer to Metal/Vulkan/D3D12. It exposes modern GPU features: compute shaders (GPU-accelerated math, not just rendering), better multi-threading, lower overhead. Available in Chrome/Edge 113+ (~70% of users as of 2025).

**`wgpu`** is Rust's GPU abstraction library. You write your rendering code once against wgpu's API, and it compiles to WebGPU when available and falls back to WebGL2 otherwise. This is how Bloom's four-tier fallback works — the same Rust code adapts at runtime.

### WGSL shaders

WGSL (*WebGPU Shading Language*) is the shader language used by WebGPU. Shaders are small programs that run on the GPU. They're included in the Rust binary via `include_str!()` at compile time and uploaded to the GPU at initialization.

---

## 9. Instanced Rendering

### The draw call problem

Every GPU draw call has overhead: the CPU must submit commands to the GPU driver, which validates them and schedules work. For 10,000 nodes, 10,000 draw calls would waste most of the frame budget on overhead.

### Instancing

*Instanced rendering* lets you draw many copies of the same geometry in a single draw call. You provide:
- **Geometry**: A quad (two triangles = four vertices) that represents a single node shape.
- **Instance data**: A buffer of per-node data (position, size, color) — one entry per node.

The GPU then runs the vertex shader once per vertex *per instance*, with each instance reading its own entry from the instance buffer. 10,000 nodes = 1 draw call.

### Why a quad for circles?

You can't directly draw circles on a GPU — only triangles. Instead, you draw a square quad and use the *fragment shader* to discard pixels outside the circle:

```wgsl
let d = length(uv);  // distance from center
let alpha = 1.0 - smoothstep(0.9, 1.0, d);
// alpha = 1.0 inside circle, fades to 0.0 at edge
```

`smoothstep` produces a smooth 0→1 transition, giving anti-aliased circle edges for free.

---

## 10. Signed Distance Field (SDF) Text

### Why text rendering is hard on the GPU

Rendering crisp text at arbitrary sizes is non-trivial. A naive approach (texture per glyph) looks blurry when scaled up or pixelated when scaled down.

### What a signed distance field is

An SDF encodes, for every pixel in a texture, the *signed distance to the nearest edge of the glyph*. Positive = outside the glyph, negative = inside.

```
[+3.2] [+2.1] [+1.0] [-0.1] [-1.2]  ← outside → inside
```

At render time, the fragment shader reads the SDF value and applies a threshold:
```wgsl
let dist = textureSample(sdf_atlas, sampler, uv).r;
let alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, dist);
```

The key insight: because you're applying a *threshold* to a smooth field, not sampling discrete pixels, **the text scales cleanly to any size from a single texture**. A 64×64 SDF texture can render legible text at sizes from 8px to 200px.

### The font atlas

Rather than one SDF texture per glyph (thousands of GPU textures), all glyphs are packed into a single *texture atlas* — one large texture containing every character. The shader reads the atlas coordinates for the requested glyph and samples the appropriate region.

---

## 11. Spatial Indexing with a Quadtree

### The hit-testing problem

When the user hovers or clicks on the canvas, you get pixel coordinates. You need to know which node (if any) the cursor is near. Naively, you check every node — O(n) per mouse event. At 10,000 nodes and 60 mouse events per second that's 600,000 comparisons per second just for hover detection.

### Quadtrees for spatial queries

The quadtree used for hit-testing (in `graph/spatial.rs`) is different from the Barnes-Hut quadtree (in `layout/barnes_hut.rs`) — they have similar structure but different purposes.

This quadtree partitions 2D space. To find nodes near a point `(x, y)`:
1. Start at the root.
2. If the current region doesn't intersect the query circle, skip it entirely.
3. If it does, check nodes in this region, then recurse into child regions.

Most of the tree is pruned per query. For uniformly distributed nodes, this reduces hit-test complexity from O(n) to **O(log n)**.

### AABB (Axis-Aligned Bounding Box)

Each quadtree region is an AABB: a rectangle aligned with the X and Y axes. AABB intersection tests are cheap (4 comparisons), which is why quadtrees use them instead of more general shapes.

---

## 12. Camera & Coordinate Systems

### Three coordinate spaces

Bloom operates in three coordinate spaces:

1. **World space**: Where nodes live. The force layout places nodes at arbitrary `(x, y)` positions in an infinite plane. No units — just numbers.
2. **View space**: Rotated/scaled world space after the camera transform. (Bloom doesn't rotate so world ≈ view.)
3. **Screen space**: Pixel coordinates on the canvas, `(0, 0)` at top-left.

The transformation from world to screen:
```
screen_x = (world_x - camera_x) * zoom + canvas_width / 2
screen_y = (world_y - camera_y) * zoom + canvas_height / 2
```

This centers the graph (camera position is what you're looking at) and applies zoom.

### The view-projection matrix

On the GPU, the vertex shader transforms positions using a *view-projection matrix* — a 4×4 matrix that encodes the camera's position, orientation, and the mapping from world space to clip space in one matrix multiply.

This is a standard technique from 3D graphics. Even for 2D rendering, using a matrix allows the vertex shader to handle arbitrary camera transformations in one operation.

### Smooth camera transitions

The `Camera::update()` method uses *exponential smoothing* (sometimes called lerp easing):

```rust
self.x += (self.target_x - self.x) * lerp_factor;
```

where `lerp_factor = 1.0 - exp(-5.0 * dt)`.

This produces the characteristic "ease out" motion: the camera moves quickly at first and slows as it approaches the target. The exponential ensures the speed is proportional to remaining distance, which gives smooth deceleration independent of frame rate.

---

## 13. The Full Pipeline

Here is how data flows from a Wikipedia database to pixels on screen:

```
Wikipedia XML dump
      ↓
   Dedalus (Rust)
   - Parses XML, extracts articles and links
   - Computes PageRank
   - Stores nodes and edges in SQLite
      ↓
   Fugue (Elixir / Phoenix LiveView)
   - Serves the web application
   - Handles search and article detail UI
   - Encodes query results as BLOM binary
   - Sends over WebSocket
      ↓
   Browser
   - Receives binary WebSocket frame
   - Passes raw bytes to Bloom (WASM)
      ↓
   Bloom Protocol Layer
   - Validates magic number and version
   - Decodes struct-of-arrays binary format
   - Constructs Node/Edge/Graph structs
      ↓
   Bloom Graph Layer
   - Builds id→index lookup map
   - Stores adjacency (edge list)
      ↓
   Bloom Layout Layer (runs every frame)
   - ForceLayout::step() computes forces
   - Barnes-Hut prunes repulsion pairs
   - SIMD paths vectorize inner loops
   - Writes new (x, y) into Node structs
      ↓
   Bloom Render Layer (runs every frame)
   - Camera::update() smooths pan/zoom
   - NodeRenderer uploads position buffer to GPU
   - GPU vertex shader transforms world → screen
   - GPU fragment shader draws antialiased circles
   - SDF text shader renders node labels
      ↓
   Canvas pixels
```

Each layer has one job and a clean interface to the next. The layout engine writes a position buffer; the renderer reads it. Neither cares about the other's implementation. This is why the four GPU tiers work: you can swap the renderer without touching the layout code.

---

## Summary

| Concept | What it is | Why Bloom uses it |
|---|---|---|
| WebAssembly | Portable binary VM in the browser | Predictable perf, no GC pauses |
| Rust | Systems language compiling to WASM | Zero-cost abstractions, no runtime |
| wasm-bindgen | Rust↔JS bridge code generator | Ergonomic API without manual pointer math |
| BLOM binary format | Packed struct-of-arrays encoding | ~10x smaller than JSON, cache-friendly |
| Force-directed layout | Physics simulation on graph | Produces human-readable layouts |
| Barnes-Hut | O(n log n) repulsion approximation | Scales to 10K+ nodes at interactive rates |
| WASM SIMD | 128-bit vectorized arithmetic | ~4x speedup on layout math, no GPU needed |
| Instanced rendering | Single draw call for N nodes | Eliminates per-node CPU→GPU overhead |
| SDF text | Distance field font atlas | Resolution-independent text at any scale |
| Quadtree (spatial) | 2D space partitioning tree | O(log n) hover/click hit-testing |
| WebGPU compute shaders | General compute on the GPU | Move layout math to GPU for Tier 1 |
| Exponential camera lerp | Smooth easing for pan/zoom | Frame-rate-independent smooth transitions |
