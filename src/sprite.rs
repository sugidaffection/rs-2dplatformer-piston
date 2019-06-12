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
	solid: bool
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
				rectangle([0.3;4], [self.pos.x, self.pos.y, 40.0, 40.0], c.transform, g);
			}
			
		});
	}
}

pub struct Player {
	key: Keypress,
	ground: bool,
	pos: Vec2D,
	scale: Vec2D,
	vel: Vec2D,
	acc: Vec2D,
	back: bool,
	animation: Vec<Sprite>,
}

impl Player {

	pub fn new() -> Player {
		Player {
			key: Keypress::new(),
			ground: false,
			pos: Vec2D::new(),
			scale: Vec2D::new(),
			vel: Vec2D::new(),
			acc: Vec2D::new(),
			back: false,
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

	pub fn update(&mut self, dt: f64, objects: &mut Vec<Object>){
		self.acc = Vec2D::new();
		self.acc.y = 0.9;
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
		if hit {
			if self.vel.y >= 0.0{
				self.pos.y = self.pos.y.round();
				self.ground = true;
			}
			
		}

		if self.key.up && self.ground{
			self.vel.y = -0.3;
			self.ground = false;
		}

		if self.pos.x + 0.3 <= 0.0 {
			self.vel.x = 0.0;
			self.pos.x = -0.3;
		}

		if self.pos.x - 0.3 >= 14.0 {
			self.vel.x = 0.0;
			self.pos.x = 14.3;
		}

		self.acc.add(self.vel.x * -2.0, 0.0);
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

	pub fn collision(&mut self, objects: &mut Vec<Object>) -> bool{
		for object in objects {
			if object.solid {

				if self.pos.x + 0.75 >= object.pos.x && self.pos.x + 0.25 <= object.pos.x + 1.0 {
					if self.pos.y <= object.pos.y + 1.0 && self.pos.y + 0.3 >= object.pos.y + 1.0 {
						self.vel.y = 0.0;
						self.key.up = false;
						return true;
					}
				}
				if self.pos.x + 0.5 >= object.pos.x && self.pos.x + 0.5 <= object.pos.x + 1.0{
					if self.pos.y + 1.1 >= object.pos.y && self.pos.y + 1.0 <= object.pos.y + 1.0{
						self.vel.y = 0.0;
						return true;
					}
				}

				if (self.pos.y + 0.1 >= object.pos.y &&
					self.pos.y <= object.pos.y + 1.0) ||
					(self.pos.y + 0.9 <= object.pos.y + 1.0 &&
					self.pos.y + 0.9 >= object.pos.y) {
					if self.pos.x + 0.8 >= object.pos.x && self.pos.x + 0.8 <= object.pos.x + 1.0 {
						self.vel.x = 0.0;
						self.pos.x = object.pos.x - 0.8;
					}

					if self.pos.x + 0.2 <= object.pos.x + 1.0 && self.pos.x + 0.2 >= object.pos.x {
						self.vel.x = 0.0;
						self.pos.x = object.pos.x + 0.8;
					}
				}

				
			}
			
		}

		false
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