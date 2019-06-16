use crate::object::Object;
use crate::player::Player;
use crate::libs::Rect;

pub enum Interact {
	left, right, top, bottom
}

pub struct Collider {
	pub interact: Option<(Interact, f64)>
}

impl Collider {
	
	pub fn collision(&mut self, player: &Rect, target: &Vec<Object>) -> bool{
		for object in target.iter(){
			if object.solid {
				if player.center().x >= object.rect.left() && player.center().x <= object.rect.right() {
					if player.bottom() >= object.rect.top() && player.top() < object.rect.top(){
						self.interact = Some((Interact::bottom, object.rect.top() - player.scale));
						return true;
					}
					if player.top() < object.rect.bottom() && player.bottom() > object.rect.bottom() {
						self.interact = Some((Interact::top, object.rect.bottom()));
						return true;
					}
				}

				if player.center().y >= object.rect.top() && player.center().y <= object.rect.bottom() {
					if player.right() >= object.rect.left() && player.right() < object.rect.right(){
						self.interact = Some((Interact::right, object.rect.left() - player.scale));
						return true;
					}

					if player.left() < object.rect.right() && player.right() > object.rect.right() {
						self.interact = Some((Interact::left, object.rect.right()));
						return true;
					}
				}
			}
		}

		false
	}

}