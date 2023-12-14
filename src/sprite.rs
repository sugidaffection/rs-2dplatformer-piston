use piston_window::*;
use std::path::PathBuf;

pub trait SpriteEvent {
    fn render(&mut self, e: &Event, w: &mut PistonWindow);
}

#[derive(Clone)]
pub struct Sprite {
    texture: Vec<G2dTexture>,
}

impl Sprite {
    pub fn load_texture(path: PathBuf, w: &mut PistonWindow, flip: Flip) -> G2dTexture {
        Texture::from_path(
            &mut w.create_texture_context(),
            path,
            flip,
            &TextureSettings::new(),
        )
        .unwrap()
    }

    pub fn load_texture_as_sprite(path: PathBuf, w: &mut PistonWindow, flip: Flip) -> Sprite {
        Sprite::new(
            Texture::from_path(
                &mut w.create_texture_context(),
                path,
                flip,
                &TextureSettings::new(),
            )
            .unwrap(),
        )
    }

    pub fn new(texture: G2dTexture) -> Sprite {
        Sprite {
            texture: vec![texture],
        }
    }

    pub fn get_texture(&self, idx: usize) -> &G2dTexture {
        &self.texture[idx]
    }

    pub fn add_texture(&mut self, texture: G2dTexture) {
        self.texture.push(texture);
    }
}
