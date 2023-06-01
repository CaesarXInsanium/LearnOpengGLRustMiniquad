use glam::*;
use miniquad::*;

pub const VERTEX_SRC: &str = include_str!("vertex.glsl");
pub const FRAG_SRC: &str = include_str!("frag.glsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AppUniforms {
    pub transform: [f32; 16],
}

pub fn to_f32_array(value: Mat4) -> [f32; 16] {
    let mut result = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    value.write_cols_to_slice(&mut result);

    result
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("uTransform", UniformType::Mat4)],
        },
        images: vec!["tex01".to_string(), "tex02".to_string()],
    }
}
