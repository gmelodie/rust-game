use crate::objects::VisualObject;
use crate::utils;

use macroquad::prelude::{
    draw_texture_ex, get_frame_time, load_texture, screen_height, screen_width, Color,
    DrawTextureParams, Texture2D, Vec2, Vec3,
};

#[derive(Clone, Copy)]
pub struct Bird {
    pub direction: Vec2,
    speed: f32,
    scale: f32,
    pub pos: Vec2,
    pub leader_direction: Vec2,
    pub leader_pos: Vec2,
    texture: Texture2D,
    color: Color,
}

pub const NUMBER_OF_BIRDS: i32 = 30;

impl Bird {
    async fn new() -> Self {
        Self {
            direction: utils::random_direction(),
            speed: 100.0,
            scale: 25.0,
            texture: load_texture("./assets/bird.png").await.unwrap(),
            leader_direction: utils::random_direction(),
            leader_pos: utils::random_pos(),
            pos: utils::random_pos(),
            color: utils::random_color(),
        }
    }
}

fn diff_directions(dir1: Vec2, dir2: Vec2) -> f32 {
    let vec3_dir1 = Vec3::new(dir1.x, dir1.y, 0.0);
    let vec3_dir2 = Vec3::new(dir2.x, dir2.y, 0.0);
    let cross_prod = vec3_dir1.cross(vec3_dir2);
    cross_prod.dot(cross_prod)
}

pub fn find_leader(birds: &Vec<Bird>) -> Bird {
    // new leader is the bird with most different direction
    // 1. caclulate average group vector (sum of all vectors)
    // 2. for every bird
    //      mod(bird.direction - average_direction)
    // 3. get the max of the above

    // calculate averade direction
    let mut average_direction = Vec2::new(0.0, 0.0);
    for bird in birds {
        average_direction += bird.direction;
    }
    average_direction = average_direction.normalize_or_zero();

    // get bird with most different direction (biggest cross product vector)
    let mut leader = &birds[0];
    let mut max_diff = 0.0;

    for bird in birds {
        let diff = diff_directions(average_direction, bird.direction);
        if diff > max_diff {
            leader = &bird;
            max_diff = diff;
        }
    }

    return *leader;
}

impl VisualObject for Bird {
    fn update(&mut self) {
        // TODO: go towards leader or go in the same direction as leader is going?
        // go towards leader
        self.direction += self.leader_direction + utils::random_direction();
        self.direction = self.direction.normalize_or_zero();
        // go in the same direction as leader
        // self.direction = self.leader_direction;
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
