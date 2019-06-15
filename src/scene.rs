use piston_window::*;

use crate::camera::Camera;
use crate::sprite::{Object, Player, Sprite, SpriteEvent};
use crate::libs::Tilemap;
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

		let mut player = Player::new(40.0);

		let ground_texture = Sprite::load_texture(assets.join("ground.png"), w, Flip::None);
		let brick_texture = Sprite::load_texture(assets.join("brick.png"), w, Flip::None);
		let brick2_texture = Sprite::load_texture(assets.join("brick2.png"), w, Flip::None);
		let cloud_texture = Sprite::load_texture(assets.join("cloud.png"), w, Flip::None);
		let player_texture = Sprite::load_texture(assets.join("player.png"), w, Flip::None);
		let player_back_texture = Sprite::load_texture(assets.join("player.png"), w, Flip::Horizontal);

		player.add_animation(player_texture);
		player.add_animation(player_back_texture);

		for (row, tiles) in tilemap.iter().enumerate() {
			for (col, tile) in tiles.iter().enumerate() {
				if *tile == 'P' {
					player.set_pos(col as f64, row as f64);
				}

				if *tile == '1' {
					let mut object = Object::new(col as f64, row as f64, 40.0);
					object.add_sprite(ground_texture.clone(), true);
					objects.push(object);
				}

				if *tile == '2' {
					let mut object = Object::new(col as f64, row as f64, 40.0);
					object.add_sprite(brick_texture.clone(), true);
					objects.push(object);
				}

				if *tile == '?' {
					let mut object = Object::new(col as f64, row as f64, 40.0);
					object.add_sprite(brick2_texture.clone(), true);
					objects.push(object);
				}

				if *tile == '@' {
					let mut object = Object::new(col as f64, row as f64, 40.0);
					object.add_sprite(cloud_texture.clone(), false);
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

		self.camera.show(e, w);

		for object in self.objects.iter_mut().filter(|o| o.pos.x.round() >= -40.0 && o.pos.x.round() <= width && o.pos.y.round() >= -40.0 && o.pos.x.round() <= height){
			object.render(e, w);
		}

		self.player.render(e, w);
		self.player.key_event(e);

		if let Some(u) = e.update_args(){
			self.player.update(u.dt, &mut self.objects);
			self.camera.update(&mut self.player, &mut self.objects);
		}

	}

}