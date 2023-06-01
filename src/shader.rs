use miniquad::*;

pub const VERTEX_SRC: &str = include_str!("vertex.glsl");
pub const FRAG_SRC: &str = include_str!("frag.glsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub offset: [f32; 2],
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
        },
        images: vec!["tex".to_string()],
    }
}
