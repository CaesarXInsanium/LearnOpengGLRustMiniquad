use core::fmt;

use bytemuck;
use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pos: [f32; 3],
    color: [f32; 3],
    tex: [f32; 2],
}

#[allow(unused)]
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

#[allow(unused)]
pub const TRIANGLE_INDICES: &[u32] = &[0, 1, 3, 1, 2, 3];

pub const CUBE_ARRAY: &[Vertex] = &[
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, -0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 1.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [1.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, 0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 0.0f32],
    },
    Vertex {
        pos: [-0.5f32, 0.5f32, -0.5f32],
        color: [0.0f32, 0.0f32, 0.0f32],
        tex: [0.0f32, 1.0f32],
    },
];

pub const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35,
];
