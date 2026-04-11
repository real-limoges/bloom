struct Uniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var sdf_atlas: texture_2d<f32>;

@group(1) @binding(1)
var sdf_sampler: sampler;

struct VertexInput {
    @location(0) quad_pos: vec2<f32>,
};

struct InstanceInput {
    @location(1) position: vec2<f32>,
    @location(2) size: vec2<f32>,
    @location(3) uv_min: vec2<f32>,
    @location(4) uv_max: vec2<f32>,
    @location(5) color: vec4<f32>,
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
        vertex.quad_pos * instance.size + instance.position,
        0.0,
        1.0,
    );
    out.clip_pos = uniforms.view_proj * world;
    // Map quad_pos from [-1,1] to [0,1] for UV interpolation
    let t = vertex.quad_pos * 0.5 + 0.5;
    out.uv = mix(instance.uv_min, instance.uv_max, t);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = textureSample(sdf_atlas, sdf_sampler, in.uv).r;
    let smoothing = fwidth(dist) * 0.5;
    let alpha = smoothstep(0.5 - smoothing, 0.5 + smoothing, dist);
    if alpha < 0.01 {
        discard;
    }
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
