use piston_window::{G2dTexture, Event, PistonWindow, image, Transformed, PressEvent, Button, Key};

use crate::libs::Vec2D;

pub struct Sprite {
	pub pos: Vec2D,
	size: Vec2D,
	scale: Vec2D,
	pub vel: Vec2D,
	pub acc: Vec2D,
	texture: G2dTexture
}

impl Sprite {


	pub fn new(pos: Vec2D, size: Vec2D, scale: Vec2D, texture: G2dTexture) -> Sprite{

		Sprite{
			pos: pos,
			size: size,
			scale: scale,
			vel: Vec2D::new(),
			acc: Vec2D::new(),
			texture: texture
		}
	}

	pub fn render(&mut self, e: &Event, w: &mut PistonWindow){

		w.draw_2d(e, |c,g,_d| {
			image(&self.texture, c.trans(self.pos.x * 40.0, self.pos.y * 40.0).scale(self.scale.x, self.scale.y).transform, g);
		});

	}

	pub fn update(&mut self, e: &Event){
		self.pos.y += 1.0 * 0.6;
	}

	pub fn collision(&mut self, objects: &mut Vec<Sprite>) -> bool{
		for object in objects {
			if self.pos.y * 40.0 + self.size.y > object.pos.y * 40.0{         
				return true
			}
		}

		false
	}

}