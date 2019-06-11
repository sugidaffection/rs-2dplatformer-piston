use std::fs::File;
use std::io::prelude::Read;

pub struct Vec2D {
	pub x: f64,
	pub y: f64
}

impl Vec2D {
	pub fn new(x: usize, y:usize)->Vec2D{
		Vec2D{
			x: x as f64,
			y: y as f64
		}
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