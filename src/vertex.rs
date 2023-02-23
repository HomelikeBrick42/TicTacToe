use memoffset::offset_of;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: cgmath::Vector2<f32>,
    pub tex_coord: cgmath::Vector2<f32>,
    pub color: cgmath::Vector3<f32>,
}

// cgmath::Vector3 doesnt implement these traits, but i know its valid
unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

impl Vertex {
    pub const ATTRIBUTES: &[wgpu::VertexAttribute] = &[
        wgpu::VertexAttribute {
            offset: offset_of!(Vertex, position) as wgpu::BufferAddress,
            shader_location: 0,
            format: wgpu::VertexFormat::Float32x2,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Vertex, tex_coord) as wgpu::BufferAddress,
            shader_location: 1,
            format: wgpu::VertexFormat::Float32x2,
        },
        wgpu::VertexAttribute {
            offset: offset_of!(Vertex, color) as wgpu::BufferAddress,
            shader_location: 2,
            format: wgpu::VertexFormat::Float32x3,
        },
    ];

    pub fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
