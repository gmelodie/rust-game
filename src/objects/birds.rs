use crate::utils;
use crate::objects::{VisualObject, RESTITUTION, EARTH_GRAVITY};

use macroquad::prelude::{Color, Vec2, Texture2D, DrawTextureParams, draw_texture_ex, load_texture, get_frame_time, screen_height, WHITE};

pub struct Bird {
    velocity: Vec2,
    texture: Texture2D,
    pos: Vec2,
    color: Color,
}

pub const NUMBER_OF_BIRDS: i32 = 50;


impl Bird {
    async fn new() -> Self {
        Self {
            velocity: Vec2::new(0.0, 0.0),
            texture: load_texture("../../assets/bird.png").await.unwrap(),
            pos: utils::random_pos(),
            color: utils::random_color(),
        }
    }
    fn draw_with_scale(&self, scale: f32) {
        draw_texture_ex(
            self.texture,
            self.pos.x,
            self.pos.y,
            self.color,
            DrawTextureParams {
                dest_size: Option::Some(Vec2::new(scale, scale)),
                source: None,
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            },
        );
    }
}

impl VisualObject for Bird {
    fn update(&mut self) { // TODO: not affected by gravity
        let gravity = Vec2::new(0.0, EARTH_GRAVITY);
        self.velocity += gravity * get_frame_time();
        // bounce
        if self.hit_ground() && self.velocity.y >= 0.0 {
            self.velocity *= -RESTITUTION;
        }
        self.pos += self.velocity * get_frame_time();
    }

    fn draw(&self) {
        self.draw_with_scale(50.0);
    }
    fn hit_ground(&self) -> bool {
        return self.pos.y + 50.0 >= screen_height();
    }
}

pub async fn gen_birds(len: i32) -> Vec<Bird> {
    let mut birds = Vec::new();
    for _ in 1..len {
        birds.push(Bird::new().await);
    }
    return birds;
}
//bird chooses where to fly to (direction)
//
//birds influence each other
// loop {
//      1. choose leader based on something
//      2. all birds fly on leader's direction
// }
//
