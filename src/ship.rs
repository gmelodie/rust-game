use crate::objects::{Drawable, Movable, Object};
use crate::projectile::Projectile;
use macroquad::prelude::{draw_poly, get_frame_time, screen_height, screen_width, Vec2, WHITE};

pub struct Ship {
    object: Object,
    rotation: f32,
}
impl Drawable for Ship {
    fn draw(&self) {
        draw_poly(
            self.object.position.x,
            self.object.position.y,
            3, // number of sides
            self.object.size,
            self.rotation, // rotation (doesn't matter)
            WHITE,
        );
    }
}
impl Movable for Ship {
    fn update(&mut self) {
        self.object.update();
    }
}
impl Ship {
    pub fn new() -> Self {
        let position = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let my_rotation = -45.0;
        let size = 25.0;
        let direction = Vec2::from_angle(my_rotation);
        Self {
            object: Object::new(position, direction, 0.0, size),
            rotation: my_rotation,
        }
    }

    pub fn rotate(&mut self, degrees: f32) {
        self.rotation += degrees;
        self.object.direction = Vec2::from_angle(self.rotation);
    }

    pub fn shoot(&self) -> Projectile {
        Projectile {
            object: Object {
                position: self.object.position,
                direction: self.object.direction,
                speed: 10.0,
                size: 10.0,
            },
        }
    }
}
