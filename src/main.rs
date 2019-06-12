extern crate piston_window;
use piston_window::*;
use find_folder;

mod libs;
mod sprite;
use libs::{TileMap, Camera};
use sprite::{Sprite, SpriteEvent, Player};

struct Game {
	tilemap: TileMap,
	ground: Vec<Sprite>,
	cloud: Vec<Sprite>,
	player: Player,
	camera: Camera
}

impl Game {

	fn new(w: &mut PistonWindow) -> Game{
		let tilemap = TileMap::new("assets/map.txt");

		let assets = find_folder::Search::Kids(1)
    				.for_folder("assets").unwrap();
		
		let ground_texture = Sprite::create_texture(assets.join("ground.png"), w, Flip::None);
		let brick_texture = Sprite::create_texture(assets.join("brick.png"), w, Flip::None);
		let brick2_texture = Sprite::create_texture(assets.join("brick2.png"), w, Flip::None);
		let cloud_texture = Sprite::create_texture(assets.join("cloud.png"), w, Flip::None);
		let player_texture = Sprite::create_texture(assets.join("player.png"), w, Flip::None);
		let player_back_texture = Sprite::create_texture(assets.join("player.png"), w, Flip::Horizontal);


		let mut ground = Vec::new();
		let mut cloud = Vec::new();
		let mut player_sprite = Sprite::new(0, 0, player_texture.clone());
		let mut player = Player::new(player_sprite);

		for (i, tiles) in tilemap.map.iter().enumerate() {
			for (j, tile) in tiles.chars().enumerate() {
				if tile == 'P' {
					player_sprite = Sprite::new(
							j, i,
							player_texture.clone()
						);
				}
				if tile == '1' {
					ground.push(
						Sprite::new(
							j, i,
							ground_texture.clone()
						)
					)
				}

				if tile == '2' {
					ground.push(
						Sprite::new(
							j,i,
							brick_texture.clone()
						)
					)
				}

				if tile == '?' {
					ground.push(
						Sprite::new(
							j,i,
							brick2_texture.clone()
						)
					)
				}

				if tile == '@' {
					cloud.push(
						Sprite::new(
							j,i,
							cloud_texture.clone()
						)
					)
				}
			}
		};

		player.texture.push(player_back_texture);
		
		Game{
			tilemap: tilemap,
			ground: ground,
			cloud: cloud,
			player: player,
			camera: Camera::new(-1.0, w.size().width / 40.0)
		}
	}

	pub fn run(&mut self, e: &Event, w: &mut PistonWindow){
			
		w.draw_2d(e, |_, g, _d | {
			clear(color::hex("aaeeffff"), g);
		});

		for ground in self.ground.iter_mut().filter(|g| g.pos.x < 15.0){
			ground.render(e, w);
		}

		for cloud in self.cloud.iter_mut().filter(|c| c.pos.x < 15.0) {
			cloud.render(e, w);
		}

		self.player.render(e, w);
		

		if let Some(u) = e.update_args(){
			self.player.update(u.dt, self.ground.as_mut());
		}

		self.player.key_press(e);
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
