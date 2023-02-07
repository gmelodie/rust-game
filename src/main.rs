use macroquad::prelude::{
    draw_poly, get_frame_time, next_frame, screen_height, screen_width, Color, Conf, Vec2,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const EARTH_GRAVITY: f32 = 1000.0;
const RESTITUTION: f32 = 0.6;
const NUMBER_OF_OBJECTS: i32 = 50;

struct Polygon {
    sides: u8,
    radius: f32,
    rotation: f32,
}

struct VisualObject {
    velocity: Vec2,
    color: Color,
    center: Vec2,
    shape: Polygon,
}

impl VisualObject {
    fn new(pos: Vec2, col: Color) -> VisualObject {
        VisualObject {
            velocity: Vec2::new(0.0, 0.0),
            center: pos,
            color: col,
            shape: Polygon {
                sides: 4,
                radius: screen_height() / 20.0, // 5% of screen
                rotation: 45.0,
            },
        }
    }
    // Returns a VisualObject on a random position and with a random shape
    fn new_rand() -> VisualObject {
        VisualObject::new(random_pos(), random_color())
    }

    fn hit_ground(&self) -> bool {
        return self.center.y + self.shape.radius >= screen_height();
    }

    fn update_pos(&mut self) {
        let gravity = Vec2::new(0.0, EARTH_GRAVITY);
        self.velocity += gravity * get_frame_time();
        // bounce
        if self.hit_ground() && self.velocity.y >= 0.0 {
            self.velocity *= -RESTITUTION;
        }
        self.center += self.velocity * get_frame_time();
    }

    fn draw(&self) {
        // TODO: more types (non-polygons), match
        draw_poly(
            self.center.x,
            self.center.y,
            self.shape.sides,
            self.shape.radius,
            self.shape.rotation,
            self.color,
        );
    }
}

fn random_pos() -> Vec2 {
    let mut rng = StdRng::from_entropy();
    let x: f32 = rng.gen_range(0.0, screen_width());
    let y: f32 = rng.gen_range(0.0, screen_height());
    return Vec2::new(x, y);
}

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    // let mut rng = StdRng::from_entropy();
    let r: f32 = rng.gen_range(0.0, 1.0);
    let g: f32 = rng.gen_range(0.0, 1.0);
    let b: f32 = rng.gen_range(0.0, 1.0);
    // let a: f32 = rng.gen_range(0.5, 1.0);
    return Color::new(r, g, b, 1.0);
}

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "StupidSquids".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

fn render(objs: &mut Vec<VisualObject>) {
    for obj in objs {
        obj.update_pos();
        obj.draw();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut objs = Vec::new();
    for _ in 1..NUMBER_OF_OBJECTS {
        objs.push(VisualObject::new_rand());
    }

    loop {
        // get user inputs
        render(&mut objs);
        next_frame().await;
    }
}
