mod input_manager;

mod objects;
mod utils;
use crate::objects::birds;
// use crate::objects::polygon;

use macroquad::prelude::{is_key_pressed, next_frame, Conf, KeyCode};

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "StupidSquids".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn render<T: objects::VisualObject>(objs: &mut Vec<T>) {
    for obj in objs {
        obj.update();
        obj.draw();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = input_manager::InputManager::new();
    // let mut polys = polygon::gen_polys(polygon::NUMBER_OF_POLYGONS);
    let mut birds = birds::gen_birds(birds::NUMBER_OF_BIRDS).await;

    input.register_key(KeyCode::A);
    loop {
        let new_bird_leader = birds::find_leader(&birds);
        for bird in birds.iter_mut() {
            bird.leader = new_bird_leader;
        }
        input.update();
        if is_key_pressed(KeyCode::Q) {
            break;
        }
        if input.debounced_is_key_down(KeyCode::A) {
            birds = birds::gen_birds(birds::NUMBER_OF_BIRDS).await; // reset position of objects
        }
        render(&mut birds);
        next_frame().await;
    }
}
