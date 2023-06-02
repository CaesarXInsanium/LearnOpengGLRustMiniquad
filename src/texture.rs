use miniquad::*;

pub const IMAGE01: &[u8] = include_bytes!("rem.png");
pub const IMAGE02: &[u8] = include_bytes!("alterjoan.png");

pub fn load_texture(ctx: &mut Context, bytes: &[u8]) -> Texture {
    let img = image::load_from_memory(bytes).expect("failed to load texture");
    let rgba8 = img.into_rgba8();
    let dim = rgba8.dimensions();

    let texture = Texture::from_rgba8(ctx, dim.0 as u16, dim.1 as u16, rgba8.into_raw().as_slice());

    texture.set_filter(ctx, FilterMode::Linear);
    texture.set_wrap(ctx, TextureWrap::Clamp);
    texture.set_filter_min_mag(ctx, FilterMode::Nearest, FilterMode::Nearest);
    return texture;
}
