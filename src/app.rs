use crate::shader;
use crate::shader::to_f32_array;
use crate::texture::*;
use crate::vertex::*;
use glam::vec3;
use glam::Mat4;
use miniquad::*;

pub struct App {
    width: i32,
    height: i32,
    pipeline: Pipeline,
    bindings: Bindings,
    uniforms: shader::AppUniforms
}

impl App {
    pub fn new(ctx: &mut Context, width: i32, height: i32) -> App {
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, TRIANGLE_VERTICES);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, TRIANGLE_INDICES);

        let rem_texture = load_texture(ctx, REM_IMAGE);
        let joan_texture = load_texture(ctx, JOAN_IMAGE);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![rem_texture, joan_texture],
        };
        let shader =
            Shader::new(ctx, shader::VERTEX_SRC, shader::FRAG_SRC, shader::meta()).unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("aPos", VertexFormat::Float3),
                VertexAttribute::new("aColor", VertexFormat::Float3),
                VertexAttribute::new("aTexCoord", VertexFormat::Float2),
            ],
            shader,
        );
        let uniforms = shader::AppUniforms::default();

        Self {
            width,
            height,
            pipeline,
            bindings,
            uniforms,
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        ctx.clear(Some((0.1, 0.2, 0.3, 1.0)), Some(0.5), Some(0));

        // ctx.begin_default_pass(Default::default());
        ctx.apply_viewport(0, 0, self.width, self.height);

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let axis = vec3(0.0, 0.0, 1.0);
        let rotate = glam::Mat4::from_axis_angle(axis, 90.0f32.to_radians());
        let scale = Mat4::from_scale(vec3(0.5f32, 0.5, 0.5));
        let transform = rotate * scale;
        self.uniforms.transform = to_f32_array(transform);
        ctx.apply_uniforms(&self.uniforms);

        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.commit_frame();
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Escape => ctx.quit(),
            _ => (),
        }
    }
}
