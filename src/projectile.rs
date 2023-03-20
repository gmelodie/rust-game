use crate::objects::{Drawable, Movable, Object};
use macroquad::prelude::{draw_circle, Vec2, WHITE};

pub struct Projectile {
    pub object: Object,
    pub dead: bool,
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
impl Projectile {
    pub fn new(my_position: Vec2, my_direction: Vec2) -> Self {
        Self {
            object: Object {
                position: my_position,
                direction: my_direction,
                speed: 10.0,
                size: 5.0,
            },
            dead: false,
        }
    }
}
