use std::fs::File;
use std::io::prelude::Read;

pub struct Vec2D {
	pub x: f64,
	pub y: f64
}

impl Vec2D {
	pub fn new(x: f64, y: f64)->Vec2D{
		Vec2D{
			x: x,
			y: y
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
	pub map: Vec<String>
}


impl TileMap {

	pub fn new(path: &str)->TileMap{
		
		let mut file = File::open(path).expect("can't open file");
		let mut content = String::new();
		file.read_to_string(&mut content).expect("can't read file");

		let mut map = Vec::new();

		for lines in content.lines(){
			map.push(lines.to_owned());
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
	pub max_x: f64,
	pub min_x: f64
}

impl Camera {
	pub fn new(min_x: f64, max_x: f64)->Camera {
		Camera{
			max_x: max_x,
			min_x: min_x
		}
	}

	pub fn mov(&mut self, x: f64){
		self.min_x += x;
		self.max_x += x;
	}
}