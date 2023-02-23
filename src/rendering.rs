use encase::{ShaderSize, UniformBuffer};
use wgpu::{include_wgsl, util::DeviceExt};

use crate::{Camera, Vertex};

pub struct RenderState {
    camera_uniform_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    circle_vertices: wgpu::Buffer,
    circle_indices: wgpu::Buffer,
    circle_index_count: usize,
    circle_render_pipeline: wgpu::RenderPipeline,
}

impl RenderState {
    pub fn new(wgpu_render_state: &eframe::egui_wgpu::RenderState) -> Self {
        let circle_shader = wgpu_render_state
            .device
            .create_shader_module(include_wgsl!("./circle_shader.wgsl"));

        let circle_vertex_data = [
            Vertex {
                position: (-0.5, 0.5).into(),
                tex_coord: (0.0, 1.0).into(),
                color: (1.0, 0.0, 0.0).into(),
            },
            Vertex {
                position: (0.5, 0.5).into(),
                tex_coord: (1.0, 1.0).into(),
                color: (1.0, 0.0, 0.0).into(),
            },
            Vertex {
                position: (0.5, -0.5).into(),
                tex_coord: (1.0, 0.0).into(),
                color: (1.0, 0.0, 0.0).into(),
            },
            Vertex {
                position: (-0.5, -0.5).into(),
                tex_coord: (0.0, 0.0).into(),
                color: (1.0, 0.0, 0.0).into(),
            },
        ];

        let circle_vertices =
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Vertex Buffer"),
                    contents: bytemuck::cast_slice(&circle_vertex_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });

        let circle_index_data = [0, 1, 2, 0, 2, 3];

        let circle_indices =
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Circle Index Buffer"),
                    contents: bytemuck::cast_slice(&circle_index_data),
                    usage: wgpu::BufferUsages::INDEX,
                });

        let circle_index_count = circle_index_data.len();

        let camera_bind_group_layout =
            wgpu_render_state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("Circle Bind Group Layout"),
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

        let circle_render_pipeline_layout =
            wgpu_render_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[&camera_bind_group_layout],
                    push_constant_ranges: &[],
                });

        let circle_render_pipeline =
            wgpu_render_state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&circle_render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &circle_shader,
                        entry_point: "vs_main",
                        buffers: &[Vertex::layout()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &circle_shader,
                        entry_point: "fs_main",
                        targets: &[Some(wgpu::ColorTargetState {
                            format: wgpu_render_state.target_format,
                            blend: Some(wgpu::BlendState {
                                color: wgpu::BlendComponent::REPLACE,
                                alpha: wgpu::BlendComponent::REPLACE,
                            }),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Cw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                });

        let camera_uniform_buffer = {
            wgpu_render_state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Camera Uniform Buffer"),
                    contents: &[0; <Camera as ShaderSize>::SHADER_SIZE.get() as _],
                    usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
                })
        };

        let camera_bind_group =
            wgpu_render_state
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Camera Bind Group"),
                    layout: &camera_bind_group_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: camera_uniform_buffer.as_entire_binding(),
                    }],
                });

        Self {
            camera_uniform_buffer,
            camera_bind_group,
            circle_vertices,
            circle_indices,
            circle_index_count,
            circle_render_pipeline,
        }
    }

    pub fn prepare(
        &mut self,
        camera: Camera,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _encoder: &mut wgpu::CommandEncoder,
    ) {
        let mut buffer = UniformBuffer::new([0; <Camera as ShaderSize>::SHADER_SIZE.get() as _]);
        buffer.write(&camera).unwrap();
        let buffer = buffer.into_inner();
        queue.write_buffer(&self.camera_uniform_buffer, 0, &buffer);
    }

    pub fn render<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.circle_render_pipeline);
        render_pass.set_bind_group(0, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.circle_vertices.slice(..));
        render_pass.set_index_buffer(self.circle_indices.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.circle_index_count as u32, 0, 0..1);
    }
}
