use crate::collider::{Collider, Interact};
use crate::libs::{Controller, Rect, Vec2d};
use crate::object::Object;
use crate::sprite::{Sprite, SpriteEvent};
use piston_window::*;

pub struct Player {
    sprite: Sprite,
    controller: Controller,
    ground: bool,
    pub rect: Rect,
    acc: Vec2d,
    pub vel: Vec2d,
    friction: f64,
    collider: Collider,
    flip: bool,
}

impl Player {
    pub fn new(sprite: Sprite, rect: Rect) -> Player {
        Player {
            sprite: sprite,
            controller: Controller {
                up: false,
                left: false,
                right: false,
            },
            ground: false,
            rect: rect,
            acc: Vec2d::new(),
            vel: Vec2d::new(),
            friction: 2.0,
            collider: Collider { interact: None },
            flip: false,
        }
    }

    pub fn update(&mut self, dt: f64, object: &mut Vec<Object>) {
        self.acc = Vec2d { x: 0.0, y: 20.0 };
        self.ground = false;

        if self.controller.left {
            self.acc.x = -10.0;
            self.flip = true;
        };
        if self.controller.right {
            self.acc.x = 10.0;
            self.flip = false;
        };

        let collision = self.collider.collision(&self.rect, &object.to_vec());

        if collision {
            if let Some((interact, pos)) = &self.collider.interact {
                match interact {
                    Interact::BOTTOM => {
                        self.ground = true;
                        self.vel.y = 0.0;
                        self.rect.y = *pos;
                    }
                    Interact::TOP => {
                        self.vel.y = 0.0;
                        self.rect.y = *pos;
                    }
                    Interact::RIGHT => {
                        self.vel.x = 0.0;
                        self.rect.x = *pos;
                    }
                    Interact::LEFT => {
                        self.vel.x = 0.0;
                        self.rect.x = *pos;
                    }
                    _ => {}
                }
            }
        }

        if self.controller.up {
            if self.ground {
                self.vel.y = -10.0;
                self.ground = false;
            };
        };

        self.acc.x += self.vel.x * -self.friction;
        self.vel.add(self.acc.x * dt, self.acc.y * dt);

        self.rect.x += self.vel.x;
        self.rect.y += self.vel.y;
    }

    pub fn key_event(&mut self, e: &Event) {
        if let Some(b) = e.press_args() {
            if let Button::Keyboard(key) = b {
                match key {
                    Key::Left => self.controller.left = true,
                    Key::Right => self.controller.right = true,
                    Key::Space => self.controller.up = true,
                    _ => {}
                }
            }
        }

        if let Some(b) = e.release_args() {
            if let Button::Keyboard(key) = b {
                match key {
                    Key::Left => self.controller.left = false,
                    Key::Right => self.controller.right = false,
                    Key::Space => self.controller.up = false,
                    _ => {}
                }
            }
        }
    }
}

impl SpriteEvent for Player {
    fn render(&mut self, e: &Event, w: &mut PistonWindow) {
        let texture = self.sprite.get_texture(if self.flip { 1 } else { 0 });
        let rect = &self.rect;
        w.draw_2d(e, |c, g, _d| {
            image(
                texture,
                c.trans(rect.x, rect.y)
                    .scale(
                        rect.scale / texture.get_width() as f64,
                        rect.scale / texture.get_height() as f64,
                    )
                    .transform,
                g,
            );
        });
    }
}
