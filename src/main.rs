extern crate piston_window;
use piston_window::*;
use find_folder;

mod libs;
mod sprite;
use libs::{Vec2D, TileMap};
use sprite::Sprite;

struct Game {
	tilemap: TileMap,
	player: Vec<Sprite>,
	ground: Vec<Sprite>,
}

impl Game {

	fn new(w: &mut PistonWindow) -> Game{
		let tilemap = TileMap::new("assets/map.txt");

		let mut ground = Vec::new();
		let mut player = Vec::new();

		let assets = find_folder::Search::Kids(1)
    				.for_folder("assets").unwrap();
		
		let ground_texture = Texture::from_path(
								&mut w.create_texture_context(),
								assets.join("ground.png"),
								Flip::None,
								&TextureSettings::new()
							).unwrap();

		let player_texture = Texture::from_path(
								&mut w.create_texture_context(),
								assets.join("mario.png"),
								Flip::None,
								&TextureSettings::new()
							).unwrap();

		let player_scale_x = 0.15;
		let player_scale_y = 0.18;
		let player_size_x = player_texture.get_width() as f64 * player_scale_x;
		let player_size_y = player_texture.get_height() as f64 * player_scale_y;

		let ground_scale_x = 0.2;
		let ground_scale_y = 0.2;
		let ground_size_x = ground_texture.get_width() as f64 * ground_scale_x;
		let ground_size_y = ground_texture.get_width() as f64 * ground_scale_y;

		
		for (i, tiles) in tilemap.map.iter().enumerate() {
			for (j, tile) in tiles.chars().enumerate() {
				if tile == 'P' {
					player.push(
						Sprite::new(
							Vec2D{x: j as f64, y: i as f64},
							Vec2D{x: player_size_x, y: player_size_y},
							Vec2D{x: player_scale_x, y: player_scale_y},
							player_texture.clone()
						)
					)
				}
				if tile == '2' {
					ground.push(
						Sprite::new(
							Vec2D{x: j as f64, y: i as f64},
							Vec2D{x: ground_size_x, y: ground_size_y},
							Vec2D{x: ground_scale_x, y: ground_scale_y},
							ground_texture.clone()
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
				clear([1.0;4], g);
			});

			for ground in &mut self.ground {
				ground.render(&e, w);
			}

			self.player[0].render(&e, w);
			if !self.player[0].collision(&mut self.ground){
				self.player[0].update(&e);
				
			}else{
				self.player[0].vel.y = 0.0;
			}
			

		}

		if let Some(b) = e.press_args(){
			
			if let Button::Keyboard(key) = b {
				match key {
					Key::Space => {
						self.player[0].vel.y -= 10.0 * 0.6;
						self.player[0].pos.y += self.player[0].vel.y;
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
	window.set_ups(60);

	let mut game = Game::new(&mut window);

	while let Some(e) = window.next() {
		game.run(&e, &mut window);
	}
}
