use crate::sprite::{Sprite, SpriteEvent};
use crate::libs::Rect;
use piston_window::*;

#[derive(Clone)]
pub struct Object {
	sprite: Sprite,
	pub solid: bool,
	pub rect: Rect,
}

impl Object {
	pub fn new(sprite: Sprite, rect: Rect, solid: bool) -> Object {
		Object {
			sprite: sprite,
			solid: solid,
			rect: rect,
		}
	}
}

impl SpriteEvent for Object {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		let texture = self.sprite.get_texture(0);
		let rect = &self.rect;
		w.draw_2d(e, |c,g,_d| {
			image(
				texture, 
				c.trans(rect.x, rect.y)
				.scale(rect.scale / texture.get_width() as f64, rect.scale / texture.get_height() as f64)
				.transform, g
			);
		});
	}
}