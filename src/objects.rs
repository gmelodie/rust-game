pub mod polygon;
pub mod birds;

pub const EARTH_GRAVITY: f32 = 1000.0;
pub const RESTITUTION: f32 = 0.6;

pub trait VisualObject {
    fn update(&mut self);
    fn draw(&self);
    fn hit_ground(&self) -> bool;
}

