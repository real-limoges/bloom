use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::graph::Graph;
use crate::render::camera::Camera;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct GpuEdgeVertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

pub struct EdgeRenderer {
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertex_count: u32,
    uniform_buffer: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl EdgeRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("edge_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/edge.wgsl").into()),
        });

        // Vertex buffer — start with capacity for 1 (size 0 is invalid)
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("edge_vertices"),
            size: std::mem::size_of::<GpuEdgeVertex>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Uniform buffer for view-projection matrix
        let uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("edge_uniforms"),
            size: 64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("edge_bind_group_layout"),
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

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("edge_bind_group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("edge_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("edge_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<GpuEdgeVertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        // position
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x2,
                            offset: 0,
                            shader_location: 0,
                        },
                        // color
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x4,
                            offset: 8,
                            shader_location: 1,
                        },
                    ],
                }],
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
                topology: wgpu::PrimitiveTopology::LineList,
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
            vertex_count: 0,
            uniform_buffer,
            bind_group,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        graph: &Graph,
        camera: &Camera,
        canvas_w: f32,
        canvas_h: f32,
    ) {
        let edge_color = [0.5, 0.5, 0.5, 0.3];
        let nodes = graph.nodes();
        let vertices: Vec<GpuEdgeVertex> = graph.edges().iter()
            .filter_map(|edge| {
                let src = &nodes[graph.node_index(edge.source)?];
                let tgt = &nodes[graph.node_index(edge.target)?];
                Some([
                    GpuEdgeVertex { position: [src.x, src.y], color: edge_color },
                    GpuEdgeVertex { position: [tgt.x, tgt.y], color: edge_color },
                ])
            })
            .flatten()
            .collect();

        let count = vertices.len() as u32;
        if count > 0 {
            let data = bytemuck::cast_slice(&vertices);
            if count != self.vertex_count {
                self.vertex_buffer =
                    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("edge_vertices"),
                        contents: data,
                        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                    });
            } else {
                queue.write_buffer(&self.vertex_buffer, 0, data);
            }
        }
        self.vertex_count = count;

        let vp = camera.view_projection_matrix(canvas_w, canvas_h);
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&vp));
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        if self.vertex_count == 0 {
            return;
        }
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertex_count, 0..1);
    }
}
