use macroquad::prelude::{
    draw_poly, get_frame_time, is_key_down, is_key_pressed, next_frame, screen_height,
    screen_width, Color, Conf, KeyCode, Vec2,
};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::time::{Duration, Instant};

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
    let r: f32 = rng.gen_range(0.0, 1.0);
    let g: f32 = rng.gen_range(0.0, 1.0);
    let b: f32 = rng.gen_range(0.0, 1.0);
    return Color::new(r, g, b, 1.0);
}

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "StupidSquids".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn render(objs: &mut Vec<VisualObject>) {
    for obj in objs {
        obj.update_pos();
        obj.draw();
    }
}

fn gen_objs(len: i32) -> Vec<VisualObject> {
    let mut objs = Vec::new();
    for _ in 1..len {
        objs.push(VisualObject::new_rand());
    }
    return objs;
}

// return a duration which is the time between two frames
fn time_since(f1: i32, f2: i32) -> Duration {
    assert!(
        f2 >= f1,
        "invalid frame order: f1 {} is not greater or equal than f2 {}",
        f1,
        f2
    );
    return Duration::from_secs_f32(get_frame_time() * ((f2 - f1) as f32));
}

// debounce_time in ms
fn debounced_is_key_down(
    keys_pressed: &mut HashMap<KeyCode, (bool, i32)>,
    key: KeyCode,
    debounce_time: Duration,
    cur_frame: i32,
) -> bool {
    let when_started_being_down = keys_pressed.get(&key).copied().unwrap_or((false, 0));

    if !is_key_down(key) {
        keys_pressed.insert(key, (false, 0)); // key is no longer or not pressed, reset
    } else if is_key_down(key) && when_started_being_down.1 == 0 {
        keys_pressed.insert(key, (true, cur_frame)); // key just got pressed
    } else {
        keys_pressed.insert(key, (true, when_started_being_down.1)); // key is still being pressed
    }

    let first_down_frame = keys_pressed[&key].1;
    if keys_pressed[&key].0 && time_since(first_down_frame, cur_frame) > debounce_time {
        keys_pressed.insert(key, (false, 0));
        return true;
    }
    return false;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut keys_pressed = HashMap::new();
    let mut objs = gen_objs(NUMBER_OF_OBJECTS);
    let mut frame_num = 0;
    loop {
        frame_num += 1;
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if debounced_is_key_down(
            &mut keys_pressed,
            KeyCode::A,
            Duration::from_millis(50),
            frame_num,
        ) {
            objs = gen_objs(NUMBER_OF_OBJECTS) // reset position of objects
        }
        render(&mut objs);
        next_frame().await;
    }
}