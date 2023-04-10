use macroquad::prelude::{draw_text, screen_height, screen_width, Color, Vec2, WHITE};
// use rand::rngs::StdRng;
// use rand::{Rng, SeedableRng};
use quad_rand as qrand;
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
    let r: f32 = qrand::gen_range(0.0, 1.0);
    let g: f32 = qrand::gen_range(0.0, 1.0);
    let b: f32 = qrand::gen_range(0.0, 1.0);
    return Color::new(r, g, b, 1.0);
}

pub fn random_vec2(min_x: f32, max_x: f32, min_y: f32, max_y: f32) -> Vec2 {
    let x: f32 = qrand::gen_range(min_x, max_x);
    let y: f32 = qrand::gen_range(min_y, max_y);
    return Vec2::new(x, y);
}

pub fn deg2rad(degrees: f32) -> f32 {
    return degrees * PI / 180.0;
}

pub fn draw_header(points: i32, lives: i32) {
    let points_text = format!("Points: {}", points).to_owned();
    draw_text(&points_text, 0.0, 50.0, 40.0, WHITE);

    let lives_text = format!("Lives: {}", lives).to_owned();
    draw_text(&lives_text, 0.0, 100.0, 40.0, WHITE);
}
