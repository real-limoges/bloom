struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexInput {
    @location(0) quad_pos: vec2<f32>,
};

struct InstanceInput {
    @location(1) world_pos: vec2<f32>,
    @location(2) size: f32,
    @location(3) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    var out: VertexOutput;
    let world = vec4<f32>(
        vertex.quad_pos * instance.size + instance.world_pos,
        0.0,
        1.0,
    );
    out.clip_pos = uniforms.view_proj * world;
    out.uv = vertex.quad_pos;
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Signed distance from the unit-circle boundary (in [-1, 1] quad space).
    let dist = length(in.uv);

    // Hard edge of the disc, antialiased by one fragment.
    let edge = 1.0 - smoothstep(0.90, 1.0, dist);
    if edge < 0.001 {
        discard;
    }

    // Radial falloff from the center — white-hot core that drops to the
    // instance color at the rim. Squared to concentrate the brightness.
    let core_mask = 1.0 - smoothstep(0.0, 0.75, dist);
    let core = core_mask * core_mask;

    // Halo just past the rim — sits inside the edge cutoff so it only
    // shows up on nodes, not as full-quad bloom.
    let halo = (1.0 - smoothstep(0.55, 0.95, dist)) * 0.35;

    // Brighten toward the core by mixing in near-white over the base color.
    let base = in.color.rgb;
    let hot = mix(base, vec3<f32>(1.0, 0.92, 0.98), core);
    let rgb = hot + base * halo;

    return vec4<f32>(rgb, in.color.a * edge);
}
