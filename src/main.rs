use miniquad::*;
mod app;
mod shader;
mod texture;
mod vertex;

fn main() {
    let width = 800;
    let height = 800;
    let conf = conf::Conf {
        window_title: "LearnOpenGL".to_string(),
        window_width: width,
        window_height: height,
        sample_count: 5,
        fullscreen: false,
        window_resizable: true,
        ..Default::default()
    };
    miniquad::start(conf, |mut ctx| Box::new(app::App::new(&mut ctx, 800, 800)));
}
