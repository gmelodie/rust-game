use macroquad::prelude::{Vec2, screen_height, screen_width, Color};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

pub fn random_pos() -> Vec2 {
    let mut rng = StdRng::from_entropy();
    let x: f32 = rng.gen_range(0.0, screen_width());
    let y: f32 = rng.gen_range(0.0, screen_height());
    return Vec2::new(x, y);
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0, 1.0);
    let g: f32 = rng.gen_range(0.0, 1.0);
    let b: f32 = rng.gen_range(0.0, 1.0);
    return Color::new(r, g, b, 1.0);
}

