extern crate piston_window;
use piston_window::*;
use find_folder;

mod libs;
mod sprite;
use libs::{TileMap};
use sprite::Sprite;

struct Game {
	tilemap: TileMap,
	player: Sprite,
	ground: Vec<Sprite>,
}

impl Game {

	fn new(w: &mut PistonWindow) -> Game{
		let tilemap = TileMap::new("assets/map.txt");

		let mut ground = Vec::new();

		let assets = find_folder::Search::Kids(1)
    				.for_folder("assets").unwrap();
		
		let ground_texture = Sprite::create_texture(assets.join("ground.png"), w);
		let brick_texture = Sprite::create_texture(assets.join("brick.png"), w);
		let brick2_texture = Sprite::create_texture(assets.join("brick2.png"), w);
		let cloud_texture = Sprite::create_texture(assets.join("cloud.png"), w);
		let player_texture = Sprite::create_texture(assets.join("player.png"), w);

		let mut player = Sprite::new(0, 0, player_texture.clone());

		for (i, tiles) in tilemap.map.iter().enumerate() {
			for (j, tile) in tiles.chars().enumerate() {
				if tile == 'P' {
					player = Sprite::new(
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
					ground.push(
						Sprite::new(
							j,i,
							cloud_texture.clone()
						)
					)
				}
			}
		}
		
		Game{
			tilemap: tilemap,
			player: player,
			ground: ground,
		}
	}

	pub fn run(&mut self, e: &Event, w: &mut PistonWindow){
		if let Some(_) = e.render_args(){
			
			w.draw_2d(e, |c, g, _d | {
				clear(color::hex("aaeeffff"), g);
			});

			for ground in &mut self.ground {
				ground.render(&e, w);
			}
			self.player.render(&e, w);
		}

		if let Some(u) = e.update_args(){
			self.player.update(u.dt, &mut self.ground);
		}

		if let Some(b) = e.press_args(){
			
			if let Button::Keyboard(key) = b {
				match key {
					Key::Space => {
						self.player.vel.y -= 3.0 * 0.6;
						self.player.pos.y += self.player.vel.y;
					},
					Key::Right => {
						self.player.vel.x += 0.1 * 0.6;
						self.player.pos.x += self.player.vel.x;
					},
					Key::Left => {
						self.player.vel.x += 0.1 * 0.6;
						self.player.pos.x -= self.player.vel.x;
					},
					_ => println!("{:?}", key)
				}
			}
		}

		if let Some(b) = e.release_args(){
			
			if let Button::Keyboard(key) = b {
				match key {
					Key::Right => {
						self.player.vel.x = 0.0;
					},
					Key::Left => {
						self.player.vel.x = 0.0;
					},
					_ => println!("{:?}", key)
				}
			}
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
	window.set_ups(120);

	let mut game = Game::new(&mut window);

	while let Some(e) = window.next() {
		game.run(&e, &mut window);
	}
}
