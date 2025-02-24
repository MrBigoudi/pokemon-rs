#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub tex_coords: glam::Vec2,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x2];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn triangle_vertices() -> Vec<Vertex> {
        vec![
            Vertex {
                position: glam::Vec3 {
                    x: 0.,
                    y: 0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 0.5, y: 0. },
            },
            Vertex {
                position: glam::Vec3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 0., y: 1. },
            },
            Vertex {
                position: glam::Vec3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 1., y: 1. },
            },
        ]
    }

    pub fn triangle_indices() -> Vec<u16>{
        vec![0,1,2]
    }

    pub fn rectangle_vertices() -> Vec<Vertex> {
        vec![
            Vertex {
                position: glam::Vec3 {
                    x: -0.5,
                    y: 0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 0., y: 0. },
            },
            Vertex {
                position: glam::Vec3 {
                    x: -0.5,
                    y: -0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 0., y: 1. },
            },
            Vertex {
                position: glam::Vec3 {
                    x: 0.5,
                    y: 0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 1., y: 0. },
            },
            Vertex {
                position: glam::Vec3 {
                    x: 0.5,
                    y: -0.5,
                    z: 0.,
                },
                tex_coords: glam::Vec2 { x: 1., y: 1. },
            },
        ]
    }

    pub fn rectangle_indices() -> Vec<u16>{
        vec![0, 1, 2, 2, 1, 3]
    }
}