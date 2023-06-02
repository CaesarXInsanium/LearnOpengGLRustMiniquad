use crate::shader;
use crate::shader::mat4_to_f32_array;
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
    uniforms: shader::AppUniforms,
    time_start: std::time::SystemTime,
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
        let time_start = std::time::SystemTime::now();

        Self {
            width,
            height,
            pipeline,
            bindings,
            uniforms,
            time_start,
        }
    }
}

impl EventHandler for App {
    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        self.width = width as i32;
        self.height = height as i32;
        ctx.apply_viewport(0, 0, self.width, self.height);
    }
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&mut self, ctx: &mut Context) {
        // apply viewport before clearing
        ctx.begin_default_pass(PassAction::Clear {
            color: Some((0.1, 0.2, 0.3, 1.0)),
            depth: Some(1.0),
            stencil: None,
        });
        ctx.apply_viewport(0, 0, self.width, self.height);

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let time = std::time::SystemTime::now();
        let delta = std::time::SystemTime::duration_since(&time, self.time_start)
            .expect("failed to calculate time")
            .as_millis() as f32
            * 0.1;
        let angle = (delta as f32).to_radians();

        let axis = vec3(0.0, 0.0, 1.0);
        let rotate = glam::Mat4::from_axis_angle(axis, angle);
        let scale = Mat4::from_scale(vec3(0.5f32, 0.5, 0.5));
        let transform = rotate * scale;
        self.uniforms.transform = mat4_to_f32_array(transform);
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
