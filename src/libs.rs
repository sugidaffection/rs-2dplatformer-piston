use std::fs::File;
use std::io::prelude::Read;

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
}

 pub struct Tilemap();

impl Tilemap {

	pub fn new(path: &str)->Vec<Vec<char>>{
		
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

		map
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