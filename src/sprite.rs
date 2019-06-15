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

	pub fn load_texture(path: PathBuf,  w: &mut PistonWindow, flip: Flip) -> Sprite{

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
	pub fn new(x: f64, y: f64, scale: f64) -> Object {
		Object {
			pos: Vec2D{x: x * scale, y: y * scale},
			scale: Vec2D{x: scale, y: scale},
			sprite: Option::None,
			solid: false
		}
	}

	pub fn add_sprite(&mut self, sprite: Sprite, solid: bool) {
		self.sprite = Some(sprite);
		self.solid = solid;
	}
}

impl SpriteEvent for Object {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		w.draw_2d(e, |c,g,_d| {
			if let Some(sprite) = &self.sprite {
				image(
					&sprite.texture, 
					c.trans(self.pos.x, self.pos.y)
					.scale(self.scale.x / sprite.texture.get_width() as f64, self.scale.y / sprite.texture.get_height() as f64)
					.transform, g
				);
			}
		});
	}
}

pub struct Player {
	key: Keypress,
	pub ground: bool,
	pub pos: Vec2D,
	pub scale: Vec2D,
	pub back: bool,
	pub lock: bool,
	acc: Vec2D,
	pub vel: Vec2D,
	animation: Vec<Sprite>,
}

impl Player {

	pub fn new(scale: f64) -> Player {
		Player {
			key: Keypress::new(),
			ground: false,
			pos: Vec2D::new(),
			scale: Vec2D{x: scale, y: scale},
			back: false,
			lock: false,
			acc: Vec2D::new(),
			vel: Vec2D::new(),
			animation: Vec::new(),
		}
	}

	pub fn add_animation(&mut self, sprite: Sprite){
		self.animation.push(sprite);
	}

	pub fn set_pos(&mut self, x: f64, y: f64) {
		self.pos.x = x * self.scale.x;
		self.pos.y = y * self.scale.y;
	}
	

	pub fn update(&mut self, dt: f64, object: &mut Vec<Object>){
		self.acc = Vec2D::new();
		self.acc.y = 20.0;
		self.ground = false;

		if self.key.right {
			self.acc.x = 10.0;
			self.back = false;
		}

		if self.key.left {
			self.acc.x = -10.0;
			self.back = true;
		}

		self.collision(object);

		if self.key.up && self.ground{
			if self.vel.y >= 0.0 {
				self.vel.y = -10.0;
				self.ground = false;
			}
			
		}

		if self.ground {
			self.vel.y = 0.0;
			self.pos.y = self.pos.y.round();
		}

		self.acc.add(self.vel.x * -2.0, 0.0);
		self.vel.add(self.acc.x * dt, self.acc.y * dt);
		
		self.pos.add(self.vel.x, self.vel.y);
	}

	pub fn collision(&mut self, objects: &mut Vec<Object>){
		for object in objects.iter_mut(){
			if object.solid {
				if self.pos.x + 20.0 >= object.pos.x && self.pos.x + 20.0 <= object.pos.x + object.scale.x{
					if self.pos.y + self.scale.y + 5.0 >= object.pos.y && self.pos.y + self.scale.y <= object.pos.y + object.scale.y{
						self.ground = true;
						self.vel.y = 0.0;
						self.pos.y = object.pos.y - self.scale.y;
					}
					if self.pos.y <= object.pos.y + object.scale.y && self.pos.y + 5.0 >= object.pos.y + object.scale.y {
						self.vel.y = 0.5;
						self.pos.y = object.pos.y + object.scale.y;
						self.key.up = false;
					}
				}

				if (self.pos.y + 5.0 >= object.pos.y &&
					self.pos.y + 5.0 <= object.pos.y + object.scale.y) ||
					(self.pos.y + self.scale.y - 5.0 <= object.pos.y + object.scale.y &&
					self.pos.y + self.scale.y - 5.0 >= object.pos.y) {
					if self.pos.x + self.scale.y - 8.0 >= object.pos.x && self.pos.x + self.scale.y - 8.0 <= object.pos.x + object.scale.x {
						self.pos.x = object.pos.x - self.scale.y + 7.0;
					}

					if self.pos.x + 8.0 <= object.pos.x + object.scale.x && self.pos.x + 8.0 >= object.pos.x {
						self.pos.x = object.pos.x + self.scale.y - 7.0;
					}
				}
			}
		}
	}

	pub fn key_event(&mut self, e: &Event){
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

}

impl SpriteEvent for Player {
	fn render(&mut self, e: &Event, w: &mut PistonWindow){
		w.draw_2d(e, |c,g,_d| {
			image(
				&self.animation[if self.back { 1 } else { 0 }].texture, 
				c.trans(self.pos.x, self.pos.y)
				.scale(self.scale.x / self.animation[0].texture.get_width() as f64, self.scale.y / self.animation[0].texture.get_height() as f64)
				.transform, g);
		});
	}
}