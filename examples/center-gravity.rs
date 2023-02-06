use macroquad::prelude::{Vec2, next_frame, BLUE, draw_circle, screen_height, screen_width, get_frame_time, Conf};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;


// const MARS_GRAVITY: f32 = 100000.0;
const EARTH_GRAVITY: f32 = 100.0;
const RESTITUTION: f32 = 0.8;

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "StupidSquids".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}


fn hit_ground(pos: Vec2) -> bool {
    return pos.y >= screen_height();
}

fn euclidean_distance(pos1: Vec2, pos2: Vec2) -> f32 {
    let diff_x: f32 = pos1.x - pos2.x;
    let diff_y: f32 = pos1.y - pos2.y;
    let diff_sum: f32 = diff_x.powf(2.0) + diff_y.powf(2.0);

    return diff_sum.sqrt();
}


// . . .
// . C .
// . . X

// X = 2 2
// C = 1 1
// G = -1 -1

// X . .
// . C .
// . . .

// X = 0 0
// C = 1 1
// G = 1 1
//
// grav = center - x

fn gravity_circle(pos: Vec2) -> Vec2 {
    let center = Vec2::new(screen_width()/2.0, screen_height()/2.0);
    return center - pos;
}


fn render(pos: &mut Vec2, velocity: &mut Vec2) {
    let down_gravity = Vec2::new(0.0, EARTH_GRAVITY);
    let gravity = gravity_circle(*pos) + down_gravity;

    *velocity += gravity*get_frame_time();

    // bounce
    if hit_ground(*pos) && velocity.y >= 0.0 {
        *velocity *= -RESTITUTION;
    }

    *pos += *velocity*get_frame_time();
}


#[macroquad::main(window_conf)]
async fn main() {

    let mut rng = StdRng::from_entropy();

    let x: f32 = rng.gen_range(0.0, screen_width());
    let y: f32 = rng.gen_range(0.0, screen_height());
    let vel_x: f32 = rng.gen_range(0.0, screen_width()/3.0);
    let vel_y: f32 = rng.gen_range(0.0, screen_height()/3.0);

    println!("x {} y {}", x, y);

    let mut pos = Vec2::new(x, y);
    let mut velocity = Vec2::new(vel_x, vel_y);

    let mut i = 0;
    loop {
        render(&mut pos, &mut velocity);
        draw_circle(pos.x, pos.y, 30.0, BLUE);
        if i%2 == 0 {
            next_frame().await;
        }
        i += 1;
    }
}
