use std::fs::File;
use std::io::prelude::Read;

pub struct Vec2d {
	pub x: f64,
	pub y: f64
}

impl Vec2d {
	pub fn new() -> Vec2d {
		Vec2d{
			x: 0.0,
			y: 0.0
		}
	}

	pub fn add(&mut self, x: f64, y: f64){
		self.x += x;
		self.y += y;
	}
}

#[derive(Clone)]
pub struct Rect{
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64,
	pub scale: f64,
}

impl Rect {
	pub fn new(x: f64, y: f64, w: f64, h: f64, scale: f64)->Rect{
		Rect{
			x: x,
			y: y,
			w: w,
			h: h,
			scale: scale
		}
	}

	pub fn left(&self) -> f64 {
		self.x + self.w
	}

	pub fn right(&self) -> f64 {
		self.x + self.scale - self.w
	}

	pub fn top(&self) -> f64 {
		self.y + self.h
	}

	pub fn bottom(&self) -> f64 {
		self.y + self.scale - self.h
	}

	pub fn center(&self) -> Vec2d {
		Vec2d{
			x: self.x + self.scale / 2.0,
			y: self.y + self.scale / 2.0
		}
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

pub struct Controller {
	pub up: bool,
	pub left: bool,
	pub right: bool
}