use crate::objects::{Drawable, Movable, Object};
use crate::utils;
use macroquad::prelude::{draw_poly, screen_height, screen_width, Color, Vec2};

pub struct Enemy {
    object: Object,
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
    pub fn new(ship_position: Vec2) -> Self {
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
                speed: 2.5,
                size: 30.0,
            },
            color: utils::random_color(),
        }
    }
    //TODO: enemy was destroyed
    //TODO: enemy reached ship
}
