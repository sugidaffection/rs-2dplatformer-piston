use piston_window::*;

use crate::libs::Vec2D;
use std::path::PathBuf;

pub struct Sprite {
	pub pos: Vec2D,
	size: Vec2D,
	scale: Vec2D,
	pub vel: Vec2D,
	pub acc: Vec2D,
	texture: G2dTexture
}

impl Sprite {


	pub fn new(x: usize, y: usize, texture: G2dTexture) -> Sprite{
		
		let pos = Vec2D::new(x, y);
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
			vel: Vec2D::new(0, 0),
			acc: Vec2D::new(0, 0),
			texture: texture
		}
	}

	pub fn create_texture(path: PathBuf,  w: &mut PistonWindow) -> G2dTexture{
		Texture::from_path(
				&mut w.create_texture_context(),
				path,
				Flip::None,
				&TextureSettings::new()
		).unwrap()
	}

	pub fn render(&mut self, e: &Event, w: &mut PistonWindow){

		w.draw_2d(e, |c,g,_d| {
			image(&self.texture, c.trans(self.pos.x * 40.0, self.pos.y * 40.0).scale(self.scale.x, self.scale.y).transform, g);
			// rectangle([0.3;4], [
			// 	self.pos.x * 40.0, self.pos.y * 40.0, self.size.x, self.size.y
			// ], c.transform, g);
		});

	}

	pub fn update(&mut self, dt: f64, objects: &mut Vec<Sprite>){

		self.acc.y = 0.5;
		self.vel.y += self.acc.y;

		let object = self.collision(objects);
		if object.is_some(){
			self.vel.y = 0.0;
			self.pos.y = object.unwrap();
		}

		self.pos.y += self.vel.y * self.acc.y;

		
	}

	pub fn collision(&mut self, objects: &mut Vec<Sprite>) -> Option<f64>{
		for object in objects {
			if self.pos.x >= object.pos.x && self.pos.x <= object.pos.x + object.pos.x + 1.0 &&
				self.pos.y < object.pos.y + 0.5{
				if self.pos.y + 2.0 >= object.pos.y{
					return Some(object.pos.y - 1.0);
				}
			}

			if self.pos.x <= 0.0 {
				return Some(0.0);
			}
			
		}

		None
	}

}