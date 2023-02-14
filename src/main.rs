mod input_manager;
mod objects;

use macroquad::prelude::{
    is_key_pressed, next_frame, Conf, KeyCode,
};

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "StupidSquids".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn render(objs: &mut Vec<objects::VisualObject>) {
    for obj in objs {
        obj.update_pos();
        obj.draw();
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = input_manager::InputManager::new();
    let mut objs = objects::gen_objs(objects::NUMBER_OF_OBJECTS);
    let mut frame_num = 0;
    loop {
        frame_num += 1;
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        if input.is_down(KeyCode::A, frame_num) {
            objs = objects::gen_objs(objects::NUMBER_OF_OBJECTS) // reset position of objects
        }
        render(&mut objs);
        next_frame().await;
    }
}
