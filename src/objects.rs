use macroquad::prelude::Vec2;

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
        self.position += self.speed * self.direction.normalize();
        self.speed -= 0.005; // speed reduces slowly
        if self.speed < 0.0 {
            self.speed = 0.0; // so that speed doesn't become negative
        }
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
