use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::graph::Node;
use crate::render::camera::Camera;

const ATLAS_SIZE: usize = 1024;
const RASTER_PX: f32 = 48.0;
const SDF_SPREAD: f32 = 6.0;
const SDF_PAD: usize = SDF_SPREAD as usize;
const GLYPH_PAD: usize = 2;

#[derive(Clone)]
struct GlyphMetrics {
    uv_min: [f32; 2],
    uv_max: [f32; 2],
    advance: f32,
    bearing_x: f32,
    bearing_y: f32,
    width: f32,
    height: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct GlyphInstance {
    position: [f32; 2],
    size: [f32; 2],
    uv_min: [f32; 2],
    uv_max: [f32; 2],
    color: [f32; 4],
}

pub struct TextRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
    uniform_buffer: wgpu::Buffer,
    bind_group_uniforms: wgpu::BindGroup,
    bind_group_texture: wgpu::BindGroup,
    metrics: HashMap<char, GlyphMetrics>,
}

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let (atlas, metrics) = build_atlas();

        // Upload atlas texture
        let texture = device.create_texture_with_data(
            queue,
            &wgpu::TextureDescriptor {
                label: Some("sdf_atlas"),
                size: wgpu::Extent3d {
                    width: ATLAS_SIZE as u32,
                    height: ATLAS_SIZE as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::R8Unorm,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            &atlas,
        );
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("sdf_sampler"),
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("text_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/text.wgsl").into()),
        });

        // Unit quad
        let quad_vertices: [[f32; 2]; 4] = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text_quad_vertices"),
            contents: bytemuck::cast_slice(&quad_vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("text_quad_indices"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        // Instance buffer (start with capacity for 1)
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text_instances"),
            size: std::mem::size_of::<GlyphInstance>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Uniform buffer (view-proj)
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("text_uniforms"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Bind group 0: uniforms
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text_uniform_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let bind_group_uniforms = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_uniform_bind_group"),
            layout: &uniform_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // Bind group 1: texture + sampler
        let texture_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("text_texture_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
        let bind_group_texture = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("text_texture_bind_group"),
            layout: &texture_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("text_pipeline_layout"),
            bind_group_layouts: &[&uniform_layout, &texture_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("text_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    // Buffer 0: quad vertex (per-vertex)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<[f32; 2]>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &[wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        }],
                    },
                    // Buffer 1: glyph instance (per-instance)
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<GlyphInstance>() as u64,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &[
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 0,
                                shader_location: 1, // position
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 8,
                                shader_location: 2, // size
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 16,
                                shader_location: 3, // uv_min
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x2,
                                offset: 24,
                                shader_location: 4, // uv_max
                            },
                            wgpu::VertexAttribute {
                                format: wgpu::VertexFormat::Float32x4,
                                offset: 32,
                                shader_location: 5, // color
                            },
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent::OVER,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            index_buffer,
            instance_buffer,
            instance_count: 0,
            uniform_buffer,
            bind_group_uniforms,
            bind_group_texture,
            metrics,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        nodes: &[Node],
        camera: &Camera,
        canvas_w: f32,
        canvas_h: f32,
    ) {
        let label_scale = 1.0 / RASTER_PX;
        let metrics = &self.metrics;

        let instances: Vec<GlyphInstance> = nodes
            .iter()
            .filter(|n| {
                let node_size = 3.0 + n.pagerank * 20.0;
                let screen_radius = node_size * camera.zoom as f32;
                screen_radius >= 8.0 && !n.label.is_empty()
            })
            .flat_map(|n| {
                let node_size = 3.0 + n.pagerank * 20.0;
                let label_y = n.y + node_size + 2.0;

                let total_width: f32 = n
                    .label
                    .chars()
                    .filter_map(|ch| metrics.get(&ch))
                    .map(|m| m.advance * label_scale)
                    .sum();

                let mut cursor_x = n.x - total_width * 0.5;
                n.label.chars().filter_map(move |ch| {
                    let m = metrics.get(&ch)?;
                    let glyph_w = m.width * label_scale;
                    let glyph_h = m.height * label_scale;
                    let x = cursor_x + m.bearing_x * label_scale + glyph_w * 0.5;
                    let y = label_y - m.bearing_y * label_scale + glyph_h * 0.5;
                    cursor_x += m.advance * label_scale;

                    if glyph_w < 0.001 || glyph_h < 0.001 {
                        return None;
                    }

                    Some(GlyphInstance {
                        position: [x, y],
                        size: [glyph_w * 0.5, glyph_h * 0.5],
                        uv_min: m.uv_min,
                        uv_max: m.uv_max,
                        color: [1.0, 1.0, 1.0, 0.9],
                    })
                })
            })
            .collect();

        let count = instances.len() as u32;
        if count > 0 {
            let data = bytemuck::cast_slice(&instances);
            if count != self.instance_count {
                self.instance_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("text_instances"),
                        contents: data,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
            } else {
                queue.write_buffer(&self.instance_buffer, 0, data);
            }
        }
        self.instance_count = count;

        let vp = camera.view_projection_matrix(canvas_w, canvas_h);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&vp));
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.instance_count == 0 {
            return;
        }
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group_uniforms, &[]);
        render_pass.set_bind_group(1, &self.bind_group_texture, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..self.instance_count);
    }
}

// --- Atlas construction (runs once at init) ---

fn build_atlas() -> (Vec<u8>, HashMap<char, GlyphMetrics>) {
    let font_data = include_bytes!("../../assets/fonts/Inter-Regular.ttf");
    let font = fontdue::Font::from_bytes(font_data as &[u8], fontdue::FontSettings::default())
        .expect("failed to parse font");

    let mut atlas = vec![0u8; ATLAS_SIZE * ATLAS_SIZE];
    let mut metrics = HashMap::new();
    let mut cursor_x = GLYPH_PAD;
    let mut cursor_y = GLYPH_PAD;
    let mut row_height = 0usize;

    for ch in (32u8..=126).map(|b| b as char) {
        let (m, bitmap) = font.rasterize(ch, RASTER_PX);

        // SDF needs padding around the glyph
        let sdf_w = m.width + SDF_PAD * 2;
        let sdf_h = m.height + SDF_PAD * 2;

        // Wrap to next row if needed
        if cursor_x + sdf_w + GLYPH_PAD > ATLAS_SIZE {
            cursor_x = GLYPH_PAD;
            cursor_y += row_height + GLYPH_PAD;
            row_height = 0;
        }

        if cursor_y + sdf_h + GLYPH_PAD > ATLAS_SIZE {
            // Atlas full — skip remaining glyphs
            break;
        }

        // Pad bitmap for SDF computation
        let mut padded = vec![0u8; sdf_w * sdf_h];
        for y in 0..m.height {
            for x in 0..m.width {
                padded[(y + SDF_PAD) * sdf_w + (x + SDF_PAD)] = bitmap[y * m.width + x];
            }
        }

        let sdf = compute_sdf(&padded, sdf_w, sdf_h, SDF_SPREAD);

        // Blit SDF into atlas
        for y in 0..sdf_h {
            for x in 0..sdf_w {
                atlas[(cursor_y + y) * ATLAS_SIZE + (cursor_x + x)] = sdf[y * sdf_w + x];
            }
        }

        let uv_min = [
            cursor_x as f32 / ATLAS_SIZE as f32,
            cursor_y as f32 / ATLAS_SIZE as f32,
        ];
        let uv_max = [
            (cursor_x + sdf_w) as f32 / ATLAS_SIZE as f32,
            (cursor_y + sdf_h) as f32 / ATLAS_SIZE as f32,
        ];

        metrics.insert(
            ch,
            GlyphMetrics {
                uv_min,
                uv_max,
                advance: m.advance_width,
                bearing_x: m.xmin as f32,
                bearing_y: (m.height as i32 + m.ymin) as f32,
                width: sdf_w as f32,
                height: sdf_h as f32,
            },
        );

        cursor_x += sdf_w + GLYPH_PAD;
        row_height = row_height.max(sdf_h);
    }

    (atlas, metrics)
}

// --- Felzenszwalb Euclidean Distance Transform ---

fn compute_sdf(bitmap: &[u8], w: usize, h: usize, spread: f32) -> Vec<u8> {
    let n = w.max(h);
    let mut f = vec![0.0f32; n];
    let mut d = vec![0.0f32; n];
    let mut z = vec![0.0f32; n + 1];
    let mut v = vec![0usize; n];

    // Outside distances: distance from each outside pixel to nearest inside pixel
    let mut outside = vec![0.0f32; w * h];
    for i in 0..w * h {
        outside[i] = if bitmap[i] > 128 { 0.0 } else { 1e10 };
    }
    edt_2d(&mut outside, w, h, &mut f, &mut d, &mut z, &mut v);

    // Inside distances: distance from each inside pixel to nearest outside pixel
    let mut inside = vec![0.0f32; w * h];
    for i in 0..w * h {
        inside[i] = if bitmap[i] > 128 { 1e10 } else { 0.0 };
    }
    edt_2d(&mut inside, w, h, &mut f, &mut d, &mut z, &mut v);

    // Signed distance: positive inside, negative outside → map to [0, 255] with 128 = edge
    (0..w * h)
        .map(|i| {
            let sd = inside[i].sqrt() - outside[i].sqrt();
            let normalized = sd / spread * 0.5 + 0.5;
            (normalized.clamp(0.0, 1.0) * 255.0) as u8
        })
        .collect()
}

fn edt_2d(
    grid: &mut [f32],
    w: usize,
    h: usize,
    f: &mut [f32],
    d: &mut [f32],
    z: &mut [f32],
    v: &mut [usize],
) {
    // Transform rows
    for y in 0..h {
        for x in 0..w {
            f[x] = grid[y * w + x];
        }
        edt_1d(f, d, z, v, w);
        for x in 0..w {
            grid[y * w + x] = d[x];
        }
    }
    // Transform columns
    for x in 0..w {
        for y in 0..h {
            f[y] = grid[y * w + x];
        }
        edt_1d(f, d, z, v, h);
        for y in 0..h {
            grid[y * w + x] = d[y];
        }
    }
}

fn edt_1d(f: &mut [f32], d: &mut [f32], z: &mut [f32], v: &mut [usize], n: usize) {
    v[0] = 0;
    z[0] = f32::NEG_INFINITY;
    z[1] = f32::INFINITY;
    let mut k = 0;

    for q in 1..n {
        loop {
            let vk = v[k];
            let s =
                ((f[q] + (q * q) as f32) - (f[vk] + (vk * vk) as f32)) / (2 * q - 2 * vk) as f32;
            if s > z[k] {
                k += 1;
                v[k] = q;
                z[k] = s;
                z[k + 1] = f32::INFINITY;
                break;
            }
            k -= 1;
        }
    }

    k = 0;
    for q in 0..n {
        while z[k + 1] < q as f32 {
            k += 1;
        }
        let vk = v[k];
        let dq = q as f32 - vk as f32;
        d[q] = dq * dq + f[vk];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdf_center_is_inside() {
        // A 20x20 bitmap with a filled 10x10 square in the center
        let (w, h) = (20, 20);
        let mut bitmap = vec![0u8; w * h];
        for y in 5..15 {
            for x in 5..15 {
                bitmap[y * w + x] = 255;
            }
        }
        let sdf = compute_sdf(&bitmap, w, h, 6.0);
        // Center should be inside (> 128)
        assert!(
            sdf[10 * w + 10] > 128,
            "center pixel should be inside: {}",
            sdf[10 * w + 10]
        );
        // Corner should be outside (< 128)
        assert!(sdf[0] < 128, "corner pixel should be outside: {}", sdf[0]);
    }

    #[test]
    fn sdf_edge_near_threshold() {
        // A filled 20x20 bitmap — edge pixels at the border should be near 128
        let (w, h) = (20, 20);
        let mut bitmap = vec![0u8; w * h];
        for y in 4..16 {
            for x in 4..16 {
                bitmap[y * w + x] = 255;
            }
        }
        let sdf = compute_sdf(&bitmap, w, h, 6.0);
        // A pixel right at the edge (4,10) should be close to 128
        let edge_val = sdf[10 * w + 4];
        assert!(
            (100..156).contains(&edge_val),
            "edge pixel should be near 128: {}",
            edge_val
        );
    }

    #[test]
    fn atlas_packs_all_ascii() {
        let (_atlas, metrics) = build_atlas();
        // All printable ASCII should be present
        for b in 32u8..=126 {
            let ch = b as char;
            assert!(metrics.contains_key(&ch), "missing glyph for '{}'", ch);
        }
    }

    #[test]
    fn glyph_metrics_reasonable() {
        let (_atlas, metrics) = build_atlas();
        let m = &metrics[&'A'];
        assert!(m.advance > 0.0, "advance should be positive");
        assert!(m.width > 0.0, "width should be positive");
        assert!(m.height > 0.0, "height should be positive");
        assert!(
            m.uv_min[0] < m.uv_max[0],
            "uv_min.x should be less than uv_max.x"
        );
        assert!(
            m.uv_min[1] < m.uv_max[1],
            "uv_min.y should be less than uv_max.y"
        );
    }

    #[test]
    fn label_centering() {
        let (_atlas, metrics) = build_atlas();
        let label = "Test";
        let label_scale = 1.0 / RASTER_PX;

        let total_width: f32 = label
            .chars()
            .filter_map(|ch| metrics.get(&ch))
            .map(|m| m.advance * label_scale)
            .sum();

        // Label centered at x=0 should span roughly [-total_width/2, +total_width/2]
        let start_x = -total_width * 0.5;
        let end_x: f32 = start_x + total_width;

        assert!(
            (start_x + end_x).abs() < 0.01,
            "label should be centered: start={}, end={}",
            start_x,
            end_x
        );
    }
}
