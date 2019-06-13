extern crate piston_window;
use piston_window::*;
use find_folder;

mod libs;
mod sprite;
use libs::{TileMap, Camera};
use sprite::{Sprite, SpriteEvent, Player, Object};

struct Game {
	camera: Camera
}

impl Game {

	fn new(w: &mut PistonWindow) -> Game{
		let tilemap = TileMap::new("assets/map.txt");

		let assets = find_folder::Search::Kids(1)
    				.for_folder("assets").unwrap();
		
		let ground_sprite = Sprite::new(assets.join("ground.png"), w, Flip::None);
		let brick_sprite = Sprite::new(assets.join("brick.png"), w, Flip::None);
		let brick2_sprite = Sprite::new(assets.join("brick2.png"), w, Flip::None);
		let cloud_sprite = Sprite::new(assets.join("cloud.png"), w, Flip::None);
		let player_sprite = Sprite::new(assets.join("player.png"), w, Flip::None);
		let player_back_sprite = Sprite::new(assets.join("player.png"), w, Flip::Horizontal);

		let mut objects = Vec::new();
		let mut player = Player::new();
		player.add_animation(player_sprite);
		player.add_animation(player_back_sprite);
		player.set_scale(40.0);

		let mut ground = Object::new(true);
		ground.add_sprite(ground_sprite);
		ground.set_scale(40.0);

		let mut brick = Object::new(true);
		brick.add_sprite(brick_sprite);
		brick.set_scale(40.0);

		let mut brick2 = Object::new(true);
		brick2.add_sprite(brick2_sprite);
		brick2.set_scale(40.0);

		let mut cloud = Object::new(false);
		cloud.add_sprite(cloud_sprite);
		cloud.set_scale(40.0);

		for i in 0..tilemap.map[0].len() {
			for j in 0..tilemap.map.len() {
				let tile = tilemap.map[j][i];
				if tile == 'P' {
					player.set_pos(i as f64, j as f64);
				}
				if tile == '1' {
					let mut object = ground.clone();
					object.set_pos(i as f64, j as f64);
					objects.push(object);
				}

				if tile == '2' {
					let mut object = brick.clone();
					object.set_pos(i as f64, j as f64);
					objects.push(object);
				}

				if tile == '?' {
					let mut object = brick2.clone();
					object.set_pos(i as f64, j as f64);
					objects.push(object);
				}

				if tile == '@' {
					let mut object = cloud.clone();
					object.set_pos(i as f64, j as f64);
					objects.push(object);
				}
				
			}
		};

		let screen_w = tilemap.map[0].len() as f64;
		let screen_h = tilemap.map.len() as f64;
		
		Game{
			camera: Camera::new(screen_w, screen_h, w.size().width/40.0, w.size().height/40.0, player, objects)
		}
	}

	pub fn run(&mut self, e: &Event, w: &mut PistonWindow){
			
		w.draw_2d(e, |_, g, _d | {
			clear(color::hex("aaeeffff"), g);
		});

		self.camera.render(e, w);
		self.camera.keyEvent(e);

		if let Some(u) = e.update_args(){
			self.camera.update(u.dt);
		}
	}

}

fn main() {
	let opengl = OpenGL::V4_1;
	let mut window: PistonWindow = WindowSettings::new("Super Mario", (600, 600))
			.exit_on_esc(true)
			.graphics_api(opengl)
			.build()
			.unwrap();
	window.set_ups(60);

	let mut game = Game::new(&mut window);

	while let Some(e) = window.next() {
		game.run(&e, &mut window);
	}
}
