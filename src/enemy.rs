use crate::objects::{Drawable, Movable, Object};
use crate::projectile::Projectile;
use crate::utils;
use macroquad::prelude::{draw_poly, screen_height, screen_width, Color, Vec2};

pub struct Enemy {
    pub object: Object,
    pub dead: bool,
    color: Color,
}
impl Drawable for Enemy {
    fn draw(&self) {
        draw_poly(
            self.object.position.x,
            self.object.position.y,
            4, // number of sides
            self.object.size,
            45.0, // rotation (doesn't matter)
            self.color,
        );
    }
}
impl Movable for Enemy {
    fn update(&mut self) {
        self.object.update();
    }
}
impl Enemy {
    pub fn new(ship_position: Vec2, points: i32) -> Self {
        // TODO: position of enemy needs to be outside of screen
        let my_position = utils::random_vec2(
            -screen_width(),
            screen_width() * 2.0,
            -screen_height(),
            screen_height() * 2.0,
        );
        let my_direction = ship_position - my_position; // enemy flies in ship's direction
        Self {
            object: Object {
                position: my_position,
                direction: my_direction,
                speed: 2.5 + points as f32 / 100.0,
                size: 30.0,
            },
            dead: false,
            color: utils::random_color(),
        }
    }

    pub fn collided(&self, projectile: &Projectile) -> bool {
        let p_max_x = self.object.position.x + self.object.size / 2.0;
        let p_min_x = self.object.position.x - self.object.size / 2.0;
        let p_max_y = self.object.position.y + self.object.size / 2.0;
        let p_min_y = self.object.position.y - self.object.size / 2.0;

        return projectile.object.position.x >= p_min_x
            && projectile.object.position.x <= p_max_x
            && projectile.object.position.y >= p_min_y
            && projectile.object.position.y <= p_max_y;
    }
}
