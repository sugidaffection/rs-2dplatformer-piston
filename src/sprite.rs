use piston_window::*;

use crate::libs::{Vec2D, Keypress};
use std::path::PathBuf;

pub trait SpriteEvent {
	fn render(&mut self, e: &Event, w: &mut PistonWindow);
}

#[derive(Clone)]
pub struct Sprite {
	texture: G2dTexture,
}

impl Sprite {

	pub fn new(path: PathBuf,  w: &mut PistonWindow, flip: Flip) -> Sprite{

		Sprite {
			texture: Texture::from_path(
				&mut w.create_texture_context(),
				path,
				flip,
				&TextureSettings::new()
			).unwrap()
		}
	}
}

#[derive(Clone)]
pub struct Object {
	pub pos: Vec2D,
	scale: Vec2D,
	sprite: Option<Sprite>,
	pub solid: bool
}

impl Object {
	pub fn new(solid: bool) -> Object {
		Object {
			pos: Vec2D::new(),
			scale: Vec2D::new(),
			sprite: Option::None,
			solid: solid
		}
	}

	pub fn add_sprite(&mut self, sprite: Sprite) {
		self.sprite = Some(sprite);
	}

	pub fn set_pos(&mut self, x: f64, y: f64){
		self.pos = Vec2D{
			x: x,
			y: y
		}
	}

	pub fn set_scale(&mut self, size: f64){
		if let Some(sprite) = &self.sprite {
			self.scale = Vec2D{
				x: size / sprite.texture.get_width() as f64,
				y: size / sprite.texture.get_height() as f64
			}
		}
	}

	pub fn update(&mut self, x: f64, y: f64){
		self.pos.add(-x, -y);
	}
}

impl SpriteEvent for Object {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		w.draw_2d(e, |c,g,_d| {
			if let Some(sprite) = &self.sprite {
				image(
					&sprite.texture, 
					c.trans(self.pos.x * 40.0, self.pos.y * 40.0)
					.scale(self.scale.x, self.scale.y)
					.transform, g
				);
			}else{
				rectangle(color::hex("aaeeffff"), [self.pos.x * 40.0, self.pos.y * 40.0, 40.0, 40.0], c.transform, g);
			}
			
		});
	}
}

pub struct Player {
	key: Keypress,
	pub ground: bool,
	pub pos: Vec2D,
	scale: Vec2D,
	pub back: bool,
	pub lock: bool,
	animation: Vec<Sprite>,
}

impl Player {

	pub fn new() -> Player {
		Player {
			key: Keypress::new(),
			ground: false,
			pos: Vec2D::new(),
			scale: Vec2D::new(),
			back: false,
			lock: false,
			animation: Vec::new(),
		}
	}

	pub fn add_animation(&mut self, sprite: Sprite){
		self.animation.push(sprite);
	}

	pub fn set_pos(&mut self, x: f64, y: f64) {
		self.pos = Vec2D{
			x: x,
			y: y
		}
	}

	pub fn set_scale(&mut self, size: f64){
		let sprite = &self.animation[0];
		self.scale = Vec2D{
			x: size / sprite.texture.get_width() as f64,
			y: size / sprite.texture.get_height() as f64
		}
	}
	

	pub fn update(&mut self, x: f64, y: f64){
		self.pos.add(x, y);
	}

}

impl SpriteEvent for Player {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		w.draw_2d(e, |c,g,_d| {
			image(
				&self.animation[if self.back { 1 } else { 0 }].texture, 
				c.trans(self.pos.x * 40.0, self.pos.y * 40.0)
				.scale(self.scale.x, self.scale.y)
				.transform, g);
		});
	}
}