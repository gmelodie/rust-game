use crate::utils;
use crate::objects::{VisualObject};
use rand::seq::SliceRandom;

use macroquad::prelude::{Color, Vec2, Texture2D, DrawTextureParams, draw_texture_ex, load_texture, get_frame_time, screen_height};

pub struct Bird {
    direction: Vec2,
    speed: f32,
    scale: f32,
    pub leader: Vec2,
    pos: Vec2,
    texture: Texture2D,
    color: Color,
}

pub const NUMBER_OF_BIRDS: i32 = 50;


impl Bird {
    async fn new() -> Self {
        Self {
            direction: Vec2::new(1.0, 1.0).normalize(),
            speed: 100.0,
            scale: 50.0,
            texture: load_texture("./assets/bird.png").await.unwrap(),
            pos: utils::random_pos(),
            leader: utils::random_pos(),
            color: utils::random_color(),
        }
    }
}

// TODO: make this function actually find a nice leader
pub fn find_leader(birds: &Vec<Bird>) -> Vec2 {
    match birds.choose(&mut rand::thread_rng()) {
        None => Vec2::new(0.0, 0.0),
        Some(leader) => leader.pos,
    }
}

impl VisualObject for Bird {
    fn update(&mut self) {
        // TODO: find leader

        // go towards leader
        self.direction = (self.leader - self.pos + utils::random_vec2().normalize_or_zero()*100.0).normalize_or_zero(); // direction must always have module 1
        self.pos += self.direction * self.speed * get_frame_time();
    }

    fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.pos.x,
            self.pos.y,
            self.color,
            DrawTextureParams {
                dest_size: Option::Some(Vec2::new(self.scale, self.scale)),
                source: None,
                rotation: 0.,
                pivot: None,
                flip_x: false,
                flip_y: false,
            },
        );
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
