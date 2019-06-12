use piston_window::*;

use crate::libs::{Vec2D, Keypress};
use std::path::PathBuf;

pub trait SpriteEvent {
	fn render(&mut self, e: &Event, w: &mut PistonWindow);
}

pub struct Sprite {
	pub pos: Vec2D,
	pub size: Vec2D,
	pub scale: Vec2D,
	pub vel: Vec2D,
	pub acc: Vec2D,
	texture: G2dTexture,
}

impl Sprite {
	pub fn new(x: usize, y: usize, texture: G2dTexture) -> Sprite{
		
		let pos = Vec2D::new(x as f64, y as f64);
		let scale = Vec2D{
			x: 40.0 / texture.get_width() as f64,
			y: 40.0 / texture.get_height() as f64
		};
		let size = Vec2D{
			x: texture.get_width() as f64 * scale.x,
			y: texture.get_height() as f64 * scale.y
		};

		Sprite{
			pos: pos,
			size: size,
			scale: scale,
			vel: Vec2D::new(0.0, 0.0),
			acc: Vec2D::new(0.0, 0.0),
			texture: texture,
		}
	}

	pub fn create_texture(path: PathBuf,  w: &mut PistonWindow, flip: Flip) -> G2dTexture{
		Texture::from_path(
				&mut w.create_texture_context(),
				path,
				flip,
				&TextureSettings::new()
		).unwrap()
	}
}

impl SpriteEvent for Sprite {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){

		w.draw_2d(e, |c,g,_d| {
			image(
				&self.texture, 
				c.trans(self.pos.x * 40.0, self.pos.y * 40.0)
				.scale(self.scale.x, self.scale.y)
				.transform, g);
			// rectangle([0.3;4], [
			// 	self.pos.x * 40.0, self.pos.y * 40.0, self.size.x, self.size.y
			// ], c.transform, g);
		});
	}
} 

pub struct Player {
	key: Keypress,
	ground: bool,
	pos: Vec2D,
	size: Vec2D,
	scale: Vec2D,
	vel: Vec2D,
	acc: Vec2D,
	back: bool,
	pub texture: Vec<G2dTexture>,
}

impl Player {

	pub fn new(sprite: Sprite) -> Player {
		let mut texture = Vec::new();
		texture.push(sprite.texture);

		Player {
			key: Keypress::new(),
			ground: false,
			pos: sprite.pos,
			size: sprite.size,
			scale: sprite.scale,
			vel: sprite.vel,
			acc: sprite.acc,
			back: false,
			texture: texture,
		}
	}

	pub fn update(&mut self, dt: f64, objects: &mut Vec<Sprite>){
		self.acc = Vec2D::new(0.0, 1.0);
		self.ground = false;
		
		if self.key.right {
			self.acc.x = 0.3;
			self.back = false;
		}

		if self.key.left {
			self.acc.x = -0.3;
			self.back = true;
		}

		let hit = self.collision(objects);
		if let Some(pos) = hit {
			if self.vel.y >= 0.0 {
				self.pos.y = pos.y;
				self.vel.y = 0.0;
				self.ground = true;
			}
			
		}

		if self.key.up && self.ground{
			self.vel.y = -0.4;
			self.ground = false;
		}

		if self.pos.x + 0.3 <= 0.0 {
			self.vel.x = 0.0;
			self.pos.x = -0.3;
		}

		self.acc.add(self.vel.x * -1.3, 0.0);
		self.vel.add(self.acc.x * dt, self.acc.y * dt);
		self.pos.add(self.vel.x, self.vel.y);

	}

	pub fn key_press(&mut self, e: &Event){
		if let Some(b) = e.press_args(){
			if let Button::Keyboard(key) = b {
				match key {
					Key::Space => self.key.up = true,
					Key::Right => self.key.right = true,
					Key::Left => self.key.left = true,
					_ => println!("{:?}", key)
				}
			}
		}

		if let Some(b) = e.release_args(){
			if let Button::Keyboard(key) = b {
				match key {
					Key::Space => self.key.up = false,
					Key::Right => self.key.right = false,
					Key::Left => self.key.left = false,
					_ => println!("{:?}", key)
				}
			}
		}
		
	}

	pub fn collision(&mut self, objects: &mut Vec<Sprite>) -> Option<Vec2D>{
		for object in objects {
			if self.pos.x + 0.5 >= object.pos.x && self.pos.x + 0.5 <= object.pos.x + 1.0{
				if self.pos.y + 1.3 >= object.pos.y && self.pos.y <= object.pos.y + 1.0{
					let pos = Vec2D{
						x: self.pos.x,
						y: object.pos.y - 1.0
					};
					return Some(pos)
				}
			}
		}

		None
	}

}

impl SpriteEvent for Player {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		w.draw_2d(e, |c,g,_d| {
			image(
				&self.texture[if self.back { 1 } else { 0 }], 
				c.trans(self.pos.x * 40.0, self.pos.y * 40.0)
				.scale(self.scale.x, self.scale.y)
				.transform, g);
			// rectangle([0.3;4], [
			// 	self.pos.x * 40.0, self.pos.y * 40.0, self.size.x, self.size.y
			// ], c.transform, g);
		});
	}
}