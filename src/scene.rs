use piston_window::*;

use crate::camera::Camera;
use crate::sprite::{Sprite, SpriteEvent};
use crate::player::Player;
use crate::object::Object;
use crate::libs::{Tilemap, Rect};
use std::path::PathBuf;

pub struct Scene {
	player: Player,
	objects: Vec<Object>,
	camera: Camera,
}

impl Scene {

	pub fn new(assets: PathBuf, w: &mut PistonWindow)->Scene{
		let tilemap = Tilemap::new(assets.join("map.txt").to_str().unwrap());
		let mut objects = Vec::new();

		let [width,height] = [w.size().width, w.size().height];
		
		let max_w = tilemap.first().unwrap().len() as f64 * 40.0;
		let max_h = tilemap.len() as f64 * 40.0;

		let ground_texture = Sprite::load_texture(assets.join("ground.png"), w, Flip::None);
		let brick_texture = Sprite::load_texture(assets.join("brick.png"), w, Flip::None);
		let brick2_texture = Sprite::load_texture(assets.join("brick2.png"), w, Flip::None);
		let cloud_texture = Sprite::load_texture(assets.join("cloud.png"), w, Flip::None);
		let player_texture = Sprite::load_texture(assets.join("player.png"), w, Flip::None);
		let player_back_texture = Sprite::load_texture(assets.join("player.png"), w, Flip::Horizontal);

		let mut player_sprite = Sprite::new(player_texture);
		player_sprite.add_texture(player_back_texture);
		let player_rect = Rect::new(0.0, 0.0, 5.0, 0.0, 40.0);
		let player = Player::new(player_sprite, player_rect);

		for (row, tiles) in tilemap.iter().enumerate() {
			for (col, tile) in tiles.iter().enumerate() {

				if *tile == '1' {
					let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
					let ground_sprite = Sprite::new(ground_texture.clone());
					let object = Object::new(ground_sprite, rect, true);
					objects.push(object);
				}

				if *tile == '2' {
					let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
					let ground_sprite = Sprite::new(brick_texture.clone());
					let object = Object::new(ground_sprite, rect, true);
					objects.push(object);
				}

				if *tile == '?' {
					let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
					let ground_sprite = Sprite::new(brick2_texture.clone());
					let object = Object::new(ground_sprite, rect, true);
					objects.push(object);
				}

				if *tile == '@' {
					let rect = Rect::new(col as f64 * 40.0, row as f64 * 40.0, 0.0, 0.0, 40.0);
					let ground_sprite = Sprite::new(cloud_texture.clone());
					let object = Object::new(ground_sprite, rect, false);
					objects.push(object);
				}

			}
		}

		let cam_x = width/2.0 - 50.0;
		let cam_y = height/2.0 - 50.0;

		let camera = Camera::new(cam_x, cam_y, 100.0, 100.0, max_w, max_h);

		Scene{
			player: player,
			objects: objects,
			camera: camera
		}
	}

	pub fn update(&mut self, e: &Event, w: &mut PistonWindow) {
		w.draw_2d(e, |_, g, _d| {
			clear(color::hex("aaeeffff"), g);
		});

		let width = w.size().width;
		let height = w.size().height;

		for object in self.objects.iter_mut().filter(|o| o.rect.x.round() >= -40.0 && o.rect.x.round() <= width && o.rect.y.round() >= -40.0 && o.rect.x.round() <= height){
			object.render(e, w);
		}

		self.player.render(e, w);
		self.player.key_event(e);

		if let Some(u) = e.update_args(){
			self.player.update(u.dt, &mut self.objects);
			self.camera.update(&mut self.player, &mut self.objects);
		}

		// e.mouse_cursor(|x, y| {
		// 	self.player.rect.x = x - self.player.rect.scale / 2.0;
		// 	self.player.rect.y = y - self.player.rect.scale / 2.0;
		// 	self.camera.x = self.player.rect.x - self.camera.w/2.0;
		// 	self.camera.y = self.player.rect.y - self.camera.h/2.0;
		// });

	}

}