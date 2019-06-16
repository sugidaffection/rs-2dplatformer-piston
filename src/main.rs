use piston_window::*;
use find_folder;
use fps_counter::FPSCounter;

mod libs;
mod sprite;
mod scene;
mod camera;
mod player;
mod object;
mod collider;
use scene::Scene;

fn main() {
	let mut window: PistonWindow = WindowSettings::new("2D Platformer", (600, 600))
			.exit_on_esc(true)
			.build()
			.unwrap();
	window.set_ups(60);

	let mut fps_counter = FPSCounter::new();

	let assets = find_folder::Search::Kids(1).for_folder("assets").unwrap();
	let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();
	let mut scene = Scene::new(assets, &mut window);

	while let Some(e) = window.next() {
		scene.update(&e, &mut window);
		let fps = format!("{} fps", fps_counter.tick().to_string());
		window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 25.0);
            text::Text::new_color([0.0, 0.0, 0.0, 1.0], 24)
			.draw(
                &fps,
                &mut glyphs,
                &c.draw_state,
                transform, g
            ).unwrap();

            glyphs.factory.encoder.flush(device);
        });
	}
}
