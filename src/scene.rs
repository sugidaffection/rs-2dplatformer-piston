use piston_window::*;

use crate::camera::Camera;
use crate::libs::{Rect, Tilemap};
use crate::object::Object;
use crate::player::Player;
use crate::sprite::{Sprite, SpriteEvent};
use core::time;
use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;
use std::thread;

#[derive(Debug)]
pub enum LoadProgress {
    Sprites,
    Objects,
    Player,
}

pub struct Scene {
    player: Option<Player>,
    objects: Vec<Object>,
    camera: Option<Camera>,
    is_loaded: bool,
    sprites: BTreeMap<String, Sprite>,
    load_progress: VecDeque<LoadProgress>,
    progress_value: f64,
    max_progress_value: f64,
    assets: PathBuf,
}

impl Scene {
    pub fn new(assets: PathBuf) -> Scene {
        Scene {
            player: None,
            objects: vec![],
            camera: None,
            is_loaded: false,
            sprites: BTreeMap::new(),
            load_progress: VecDeque::from([
                LoadProgress::Sprites,
                LoadProgress::Objects,
                LoadProgress::Player,
            ]),
            progress_value: 0.0,
            max_progress_value: 3.0,
            assets,
        }
    }

    fn load(&mut self, w: &mut PistonWindow) {
        if let Some(state) = self.load_progress.pop_front() {
            match state {
                LoadProgress::Sprites => {
                    let assets = &self.assets;
                    let ground_sprite =
                        Sprite::load_texture_as_sprite(assets.join("ground.png"), w, Flip::None);
                    let brick_sprite =
                        Sprite::load_texture_as_sprite(assets.join("brick.png"), w, Flip::None);
                    let brick2_sprite =
                        Sprite::load_texture_as_sprite(assets.join("brick2.png"), w, Flip::None);
                    let cloud_sprite =
                        Sprite::load_texture_as_sprite(assets.join("cloud.png"), w, Flip::None);
                    let mut player_sprite =
                        Sprite::load_texture_as_sprite(assets.join("player.png"), w, Flip::None);
                    let player_texture_flip =
                        Sprite::load_texture(assets.join("player.png"), w, Flip::Horizontal);
                    player_sprite.add_texture(player_texture_flip);

                    self.sprites.insert("ground".to_owned(), ground_sprite);
                    self.sprites.insert("brick".to_owned(), brick_sprite);
                    self.sprites.insert("brick2".to_owned(), brick2_sprite);
                    self.sprites.insert("cloud".to_owned(), cloud_sprite);
                    self.sprites.insert("player".to_owned(), player_sprite);
                    self.progress_value += 1.0;
                }
                LoadProgress::Objects => {
                    let tilemap = Tilemap::new(self.assets.join("map.txt").to_str().unwrap());
                    let objects = &mut self.objects;

                    for (row, tiles) in tilemap.iter().enumerate() {
                        for (col, tile) in tiles.iter().enumerate() {
                            if let Some(ground_sprite) = self.sprites.get("ground") {
                                if *tile == '1' {
                                    let rect = Rect::new(
                                        col as f64 * 40.0,
                                        row as f64 * 40.0,
                                        0.0,
                                        0.0,
                                        40.0,
                                    );
                                    let object = Object::new(ground_sprite.clone(), rect, true);
                                    objects.push(object);
                                }
                            }

                            if let Some(brick_sprite) = self.sprites.get("brick") {
                                if *tile == '2' {
                                    let rect = Rect::new(
                                        col as f64 * 40.0,
                                        row as f64 * 40.0,
                                        0.0,
                                        0.0,
                                        40.0,
                                    );
                                    let object = Object::new(brick_sprite.clone(), rect, true);
                                    objects.push(object);
                                }
                            }

                            if let Some(brick2_sprite) = self.sprites.get("brick2") {
                                if *tile == '?' {
                                    let rect = Rect::new(
                                        col as f64 * 40.0,
                                        row as f64 * 40.0,
                                        0.0,
                                        0.0,
                                        40.0,
                                    );
                                    let object = Object::new(brick2_sprite.clone(), rect, true);
                                    objects.push(object);
                                }
                            }

                            if let Some(cloud_sprite) = self.sprites.get("cloud") {
                                if *tile == '@' {
                                    let rect = Rect::new(
                                        col as f64 * 40.0,
                                        row as f64 * 40.0,
                                        0.0,
                                        0.0,
                                        40.0,
                                    );
                                    let object = Object::new(cloud_sprite.clone(), rect, false);
                                    objects.push(object);
                                }
                            }
                        }
                    }

                    let size = w.size();

                    let [width, height] = [size.width, size.height];

                    let max_w = tilemap.first().unwrap().len() as f64 * 40.0;
                    let max_h = tilemap.len() as f64 * 40.0;

                    let cam_x = width / 2.0 - 50.0;
                    let cam_y = height / 2.0 - 50.0;

                    self.camera = Some(Camera::new(cam_x, cam_y, 100.0, 100.0, max_w, max_h));
                    self.progress_value += 1.0;
                }
                LoadProgress::Player => {
                    if let Some(player_sprite) = self.sprites.get("player") {
                        let player_rect = Rect::new(0.0, 0.0, 5.0, 0.0, 40.0);
                        self.player = Some(Player::new(player_sprite.clone(), player_rect));
                        self.progress_value += 1.0;
                    }
                }
            }
        } else {
            self.is_loaded = true;
        }

        if self.progress_value < self.max_progress_value {
            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn update(&mut self, e: &Event, w: &mut PistonWindow, glyphs: &mut Glyphs) {
        w.draw_2d(e, |_, g, _d| {
            clear(color::hex("aaeeffff"), g);
        });
        let width = w.size().width;
        let height = w.size().height;
        if self.is_loaded {
            let player = self.player.as_mut().unwrap();
            let camera = self.camera.as_mut().unwrap();

            for object in self.objects.iter_mut().filter(|o| {
                o.rect.x.round() >= -40.0
                    && o.rect.x.round() <= width
                    && o.rect.y.round() >= -40.0
                    && o.rect.x.round() <= height
            }) {
                object.render(e, w);
            }

            player.render(e, w);
            player.key_event(e);

            if let Some(u) = e.update_args() {
                player.update(u.dt, &mut self.objects);
                camera.update(player, &mut self.objects);
            }
        } else {
            w.draw_2d(e, |c, g, d| {
                let load_percentage = self.progress_value / self.max_progress_value * 100.0;
                let font_size = 48;
                let loading_str = format!("Loading {}%", load_percentage as i8);
                let loading_text = text::Text::new_color([1.0, 1.0, 1.0, 1.0], font_size).round();
                let text_width = glyphs.width(font_size, loading_str.as_str()).unwrap();
                loading_text
                    .draw(
                        loading_str.as_str(),
                        glyphs,
                        &c.draw_state,
                        c.transform
                            .trans((width - text_width) / 2.0, height / 2.0 - font_size as f64),
                        g,
                    )
                    .unwrap();

                let loading_progress_border_height = 40.0;
                let loading_progress_height = 30.0;

                let rect = Rectangle::new_border([1.0, 1.0, 1.0, 1.0], 1.0);
                rect.draw(
                    [
                        (width - loading_progress_border_height / 2.0) / 4.0,
                        (height - loading_progress_border_height) / 2.0,
                        (width + loading_progress_border_height / 2.0) / 2.0,
                        loading_progress_border_height,
                    ],
                    &c.draw_state,
                    c.transform,
                    g,
                );

                let in_rect = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
                in_rect.draw(
                    [
                        width / 4.0,
                        (height - loading_progress_height) / 2.0,
                        width / 2.0 * self.progress_value / self.max_progress_value,
                        loading_progress_height,
                    ],
                    &c.draw_state,
                    c.transform,
                    g,
                );
                glyphs.factory.encoder.flush(d);
            });
            if let Some(_) = e.idle_args() {
                self.load(w);
            }
        }

        // e.mouse_cursor(|x, y| {
        // 	self.player.rect.x = x - self.player.rect.scale / 2.0;
        // 	self.player.rect.y = y - self.player.rect.scale / 2.0;
        // 	self.camera.x = self.player.rect.x - self.camera.w/2.0;
        // 	self.camera.y = self.player.rect.y - self.camera.h/2.0;
        // });
    }
}
