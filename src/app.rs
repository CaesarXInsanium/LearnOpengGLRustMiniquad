use crate::shader;
use crate::shader::mat4_to_f32_array;
use crate::texture::*;
use crate::vertex::*;
use glam::vec3;
use miniquad::*;
use miniquad::native::gl as GL;

pub struct App {
    width: i32,
    height: i32,
    pipeline: Pipeline,
    index_count: i32,
    bindings: Bindings,
    uniforms: shader::AppUniforms,
    time_start: std::time::SystemTime,
}

impl App {
    pub fn new(ctx: &mut Context, width: i32, height: i32) -> App {
        // OpenGL flags
        unsafe{
            GL::glEnable(GL::GL_DEPTH_TEST);
        }
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, CUBE_ARRAY);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, CUBE_INDICES);

        let rem_texture = load_texture(ctx, IMAGE01);
        let joan_texture = load_texture(ctx, IMAGE02);

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
        index_count: CUBE_INDICES.len() as i32,
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
    // peform uniform calculations from here
    fn update(&mut self, ctx: &mut Context) {
        let now = std::time::SystemTime::now();
        let duration = now.duration_since(self.time_start).unwrap();
        let delta = (duration.as_millis() as f32) * 0.1;

        let model = glam::Mat4::from_axis_angle(vec3(0.5, 1.0, 0.0), delta.to_radians());

        let view = glam::Mat4::from_translation(vec3(0.0, 0.0, -3.0));

        let projection = glam::Mat4::perspective_rh(
            45.0f32.to_radians(),
            (self.width as f32) / (self.height as f32),
            0.1,
            100.0,
        );

        self.uniforms.model = mat4_to_f32_array(model);
        self.uniforms.view = mat4_to_f32_array(view);
        self.uniforms.projection = mat4_to_f32_array(projection);
    }

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

        // apply uniforms inside of the render function
        ctx.apply_uniforms(&self.uniforms);

        ctx.draw(0, self.index_count, 1);
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
