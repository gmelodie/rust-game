use crate::utils;
use crate::objects::{VisualObject, RESTITUTION, EARTH_GRAVITY};

use macroquad::prelude::{
    draw_poly, get_frame_time, screen_height,
    Color, Vec2,
};


pub const NUMBER_OF_POLYGONS: i32 = 50;


pub struct Polygon {
    sides: u8,
    radius: f32,
    rotation: f32,
    velocity: Vec2,
    color: Color,
    center: Vec2,
}

impl Polygon {
    fn new(pos: Vec2, col: Color) -> Polygon {
        Polygon {
            velocity: Vec2::new(0.0, 0.0),
            center: pos,
            color: col,
            sides: 4,
            radius: screen_height() / 20.0, // 5% of screen
            rotation: 45.0,
        }
    }
    // Returns a Polygon on a random position
    fn new_rand() -> Polygon {
        Polygon::new(utils::random_pos(), utils::random_color())
    }

}

impl VisualObject for Polygon {
    fn update(&mut self) {
        let gravity = Vec2::new(0.0, EARTH_GRAVITY);
        self.velocity += gravity * get_frame_time();
        // bounce
        if self.hit_ground() && self.velocity.y >= 0.0 {
            self.velocity *= -RESTITUTION;
        }
        self.center += self.velocity * get_frame_time();
    }

    fn draw(&self) {
        // TODO: more types (non-polygons), match
        draw_poly(
            self.center.x,
            self.center.y,
            self.sides,
            self.radius,
            self.rotation,
            self.color,
        );
    }
    fn hit_ground(&self) -> bool {
        return self.center.y + self.radius >= screen_height();
    }

}

pub fn gen_polys(len: i32) -> Vec<Polygon> {
    let mut polys = Vec::new();
    for _ in 1..len {
        polys.push(Polygon::new_rand());
    }
    return polys;
}
