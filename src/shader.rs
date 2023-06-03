use glam::*;
use miniquad::*;

pub const VERTEX_SRC: &str = include_str!("vertex.glsl");
pub const FRAG_SRC: &str = include_str!("frag.glsl");

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct AppUniforms {
    pub model: [f32; 16],
    pub view: [f32; 16],
    pub projection: [f32; 16],
}

pub fn mat4_to_f32_array(value: Mat4) -> [f32; 16] {
    let mut result = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    ];
    value.write_cols_to_slice(&mut result);

    result
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![
                UniformDesc::new("model", UniformType::Mat4),
                UniformDesc::new("view", UniformType::Mat4),
                UniformDesc::new("projection", UniformType::Mat4),
            ],
        },
        images: vec!["tex01".to_string(), "tex02".to_string()],
    }
}
