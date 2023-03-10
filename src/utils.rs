use macroquad::prelude::{screen_height, screen_width, Color, Vec2};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f32::consts::PI;

pub fn random_pos() -> Vec2 {
    return random_vec2(
        screen_width() / 4.0,
        screen_width() / 2.0,
        screen_height() / 4.0,
        screen_height() / 2.0,
    );
}

pub fn random_direction() -> Vec2 {
    return random_vec2(
        -screen_width(),
        screen_width(),
        -screen_height(),
        screen_height(),
    )
    .normalize_or_zero();
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0, 1.0);
    let g: f32 = rng.gen_range(0.0, 1.0);
    let b: f32 = rng.gen_range(0.0, 1.0);
    return Color::new(r, g, b, 1.0);
}

pub fn random_vec2(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Vec2 {
    let mut rng = StdRng::from_entropy();
    let x: f32 = rng.gen_range(min_x, max_x);
    let y: f32 = rng.gen_range(min_y, max_y);
    return Vec2::new(x, y);
}

pub fn deg2rad(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}
