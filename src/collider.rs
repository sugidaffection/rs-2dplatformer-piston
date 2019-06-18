use crate::object::Object;
use crate::player::Player;
use crate::libs::Rect;

pub enum Interact {
	LEFT, RIGHT, TOP, BOTTOM
}

pub struct Collider {
	pub interact: Option<(Interact, f64)>
}

impl Collider {
	
	pub fn collision(&mut self, player: &Rect, target: &Vec<Object>) -> bool{
		for object in target.iter(){
			if object.solid {
				if player.right() >= object.rect.left() &&
					player.left() <= object.rect.right() &&
					player.bottom() >= object.rect.top() &&
					player.top() <= object.rect.bottom() {
						let x = player.center().x - object.rect.center().x;
						let y= player.center().y - object.rect.center().y;

						if y * y > x * x {
							if y > 0.0 {
								self.interact = Some((Interact::TOP, object.rect.bottom()));
								return true;
							}else{
								self.interact = Some((Interact::BOTTOM, object.rect.top() - player.scale));
								return true;
							}
						}else{
							if x > 0.0 {
								self.interact = Some((Interact::LEFT, object.rect.right()));
								return true;
							}else{
								self.interact = Some((Interact::RIGHT, object.rect.left() - player.scale));
								return true;
							}
						}
				}
			}
		}

		false
	}

}