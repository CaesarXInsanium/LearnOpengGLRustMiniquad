use crate::shader;
use crate::texture::REM_IMAGE;
use crate::vertex::*;
use image;
use miniquad::*;

pub struct App {
    width: i32,
    height: i32,
    pipeline: Pipeline,
    bindings: Bindings,
}

impl App {
    pub fn new(ctx: &mut Context, width: i32, height: i32) -> App {
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, TRIANGLE_VERTICES);
        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, TRIANGLE_INDICES);

        let rem_img = image::load_from_memory(REM_IMAGE).expect("failed to load rem image");
        let rem_rgba8 = rem_img.to_rgba8();
        let rem_dim = rem_rgba8.dimensions();
        let rem_texture = Texture::from_rgba8(
            ctx,
            rem_dim.0 as u16,
            rem_dim.1 as u16,
            rem_rgba8.into_raw().as_slice(),
        );
        rem_texture.set_filter(ctx, FilterMode::Nearest);
        rem_texture.set_wrap(ctx, TextureWrap::Clamp);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![rem_texture],
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

        Self { width, height, pipeline, bindings }
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
        for i in 0..10 {
        let t = date::now();
            let t = t + i as f64 * 0.3;
            let offset = [t.sin() as f32 * 0.5, (t * 3.0).cos() as f32 * 0.5];
            ctx.apply_uniforms(&shader::Uniforms {
                offset: offset,
            });
            ctx.draw(0, 6, 1);
        }
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
