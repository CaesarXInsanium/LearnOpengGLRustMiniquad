use bytemuck;
use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
    tex: [f32; 2],
}

pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {
        pos: [0.5, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
        tex: [1.0, 1.0],
    }, // top right
    Vertex {
        pos: [0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
        tex: [1.0, 0.0],
    }, // bottom right
    Vertex {
        pos: [-0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
        tex: [0.0, 0.0],
    }, // bottom left
    Vertex {
        pos: [-0.5, 0.5, 0.0],
        color: [1.0, 1.0, 0.0],
        tex: [0.0, 1.0],
    }, // top left
];

pub const TRIANGLE_INDICES: &[u32] = &[0, 1, 3, 1, 2, 3];
