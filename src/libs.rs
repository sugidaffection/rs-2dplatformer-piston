use std::fs::File;
use std::io::prelude::Read;
use crate::sprite::{Object, Player, SpriteEvent};
use piston_window::*;

#[derive(Clone)]
pub struct Vec2D {
	pub x: f64,
	pub y: f64
}

impl Vec2D {
	pub fn new()->Vec2D{
		Vec2D{
			x: 0.0,
			y: 0.0
		}
	}

	pub fn add(&mut self, x: f64, y: f64) {
		self.x += x;
		self.y += y;
	}

	pub fn mult(&mut self, x: f64, y: f64) {
		self.x *= x;
		self.y *= y;
	}
}


pub struct TileMap {
	pub map: Vec<Vec<char>>
}


impl TileMap {

	pub fn new(path: &str)->TileMap{
		
		let mut file = File::open(path).expect("can't open file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("can't read file");

		let mut map = Vec::new();

		for lines in content.lines(){
			let mut row = Vec::new();
			for c in lines.chars(){
				row.push(c);
			}

			map.push(row);
		}
		
		TileMap{
			map: map
		}
	}

}

pub struct Keypress {
	pub up: bool,
	pub down: bool,
	pub left: bool,
	pub right: bool
}

impl Keypress {
	pub fn new()->Keypress{
		Keypress {
			up: false,
			down: false,
			left: false,
			right: false
		}
	}
}

pub struct Camera {
	pub screen_w: f64,
	pub screen_h: f64,
	pub w: f64,
	pub h: f64,
	pub player: Player,
	pub object: Vec<Object>,
	pub acc: Vec2D,
	pub vel: Vec2D,
	key: Keypress
}

impl Camera {
	pub fn new(screen_w: f64, screen_h: f64, w: f64, h: f64, player: Player, object: Vec<Object>)->Camera {
		Camera{
			screen_w: screen_w,
			screen_h: screen_h,
			w: w,
			h: h,
			player: player,
			object: object,
			acc: Vec2D::new(),
			vel: Vec2D::new(),
			key: Keypress::new()
		}
	}

	pub fn render(&mut self, e: &Event, w: &mut PistonWindow){
		
		let width = self.w;
		for object in self.object.iter_mut().filter(|o| o.pos.x.round() >= -1.0 && o.pos.x.round() <= width) {
			object.render(e, w);
		}

		self.player.render(e, w);
	}

	pub fn update(&mut self, dt: f64){

		self.player.pos.x = self.w / 2.0 - 0.5;
		self.player.lock = true;

		self.acc = Vec2D::new();
		self.acc.y = 0.9;
		self.player.ground = false;

		if self.key.right {
			self.acc.x = 0.3;
			self.player.back = false;
		}

		if self.key.left {
			self.acc.x = -0.3;
			self.player.back = true;
		}

		self.collision();
		

		if self.key.up && self.player.ground{
			self.vel.y = -0.3;
			self.player.ground = false;
		}
		

		self.acc.add(self.vel.x * -2.0, 0.0);
		self.vel.add(self.acc.x * dt, self.acc.y * dt);

		if self.player.lock {
			for object in self.object.iter_mut() {
				object.update(self.vel.x, 0.0);
			}
			self.player.update(0.0, self.vel.y);
		}else{
			self.player.update(self.vel.x, self.vel.y);
		}
		

	}

	pub fn collision(&mut self){
		for object in self.object.iter_mut(){
			if object.solid {
				if self.player.pos.x + 0.75 >= object.pos.x && self.player.pos.x + 0.25 <= object.pos.x + 1.0 {
					if self.player.pos.y <= object.pos.y + 1.0 && self.player.pos.y + 0.5 >= object.pos.y + 1.0 {
						self.vel.y = 0.0;
						self.key.up = false;
					}
				}
				if self.player.pos.x + 0.5 >= object.pos.x && self.player.pos.x + 0.5 <= object.pos.x + 1.0{
					if self.player.pos.y + 1.1 >= object.pos.y && self.player.pos.y + 1.0 <= object.pos.y + 1.0{
						self.vel.y = 0.0;
						self.player.pos.y = self.player.pos.y.round();
						self.player.ground = true;
					}
				}

				if (self.player.pos.y + 0.2 >= object.pos.y &&
					self.player.pos.y + 0.2 <= object.pos.y + 1.0) ||
					(self.player.pos.y + 0.8 <= object.pos.y + 1.0 &&
					self.player.pos.y + 0.8 >= object.pos.y) {
					if self.player.pos.x + 0.8 >= object.pos.x && self.player.pos.x + 0.3 <= object.pos.x + 1.0 {
						self.acc.x = 0.0;
						self.vel.x = -0.02;
					}

					if self.player.pos.x + 0.2 <= object.pos.x + 1.0 && self.player.pos.x + 0.3 >= object.pos.x {
						self.acc.x = 0.0;
						self.vel.x = 0.02;
					}
				}
			}
		}
	}

	pub fn keyEvent(&mut self, e: &Event){
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