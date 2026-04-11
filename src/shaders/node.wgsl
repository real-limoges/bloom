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
    let dist = length(in.uv);
    let alpha = 1.0 - smoothstep(0.85, 1.0, dist);
    if alpha < 0.01 {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
