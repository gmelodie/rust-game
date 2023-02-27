use crate::objects::{Drawable, Movable, Object};
use macroquad::prelude::{draw_circle, WHITE};

pub struct Projectile {
    pub object: Object,
}

impl Drawable for Projectile {
    fn draw(&self) {
        draw_circle(
            self.object.position.x,
            self.object.position.y,
            self.object.size,
            WHITE,
        );
    }
}
impl Movable for Projectile {
    fn update(&mut self) {
        self.object.update();
    }
}
