use crate::objects::{Drawable, Movable, Object};
use crate::projectile::Projectile;
use crate::utils::deg2rad;
use macroquad::prelude::{draw_poly, screen_height, screen_width, Vec2, WHITE};
use std::f32::consts::PI;

pub struct Ship {
    pub object: Object,
    degrees_rotation: f32,
}
impl Drawable for Ship {
    fn draw(&self) {
        draw_poly(
            self.object.position.x,
            self.object.position.y,
            3, // number of sides
            self.object.size,
            self.degrees_rotation,
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
        let size = 25.0;
        let direction = Vec2::from_angle(PI);
        Self {
            object: Object::new(position, direction, 0.0, size),
            degrees_rotation: 180.0,
        }
    }

    pub fn rotate(&mut self, degrees: f32) {
        self.degrees_rotation += degrees;
        self.object.direction = Vec2::from_angle(deg2rad(self.degrees_rotation));
    }

    pub fn shoot(&self) -> Projectile {
        Projectile::new(self.object.position, self.object.direction)
    }
}
