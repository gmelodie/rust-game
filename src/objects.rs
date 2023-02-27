use macroquad::prelude::{
    draw_poly, get_frame_time, screen_height, screen_width, Color, Vec2, WHITE,
};

pub const EARTH_GRAVITY: f32 = 1000.0;
pub const RESTITUTION: f32 = 0.8;

pub trait Movable {
    fn update(&mut self);
}
pub trait Drawable {
    fn draw(&self);
}
pub trait Renderable: Movable + Drawable {}
impl<T> Renderable for T where T: Movable + Drawable {}

pub struct Object {
    pub position: Vec2,
    pub direction: Vec2,
    pub speed: f32,
    pub size: f32, // scale
}

impl Movable for Object {
    fn update(&mut self) {
        self.position += self.speed * self.direction;
    }
}
impl Object {
    pub fn new(my_position: Vec2, my_direction: Vec2, my_speed: f32, my_size: f32) -> Self {
        Self {
            position: my_position,
            direction: my_direction,
            speed: my_speed,
            size: my_size,
        }
    }
}
