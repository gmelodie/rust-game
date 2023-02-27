mod input_manager;

mod objects;
mod projectile;
mod ship;
mod utils;

use ship::Ship;

use macroquad::prelude::{get_frame_time, is_key_down, is_key_pressed, next_frame, Conf, KeyCode};

// make window fullscreen
fn window_conf() -> Conf {
    Conf {
        window_title: "Killer Triangle".to_owned(),
        fullscreen: false,
        ..Default::default()
    }
}

fn render<T: objects::Renderable>(objs: &mut Vec<T>) {
    for obj in objs {
        obj.update();
        obj.draw();
    }
}
fn render_one<T: objects::Renderable>(obj: &mut T) {
    obj.update();
    obj.draw();
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut input = input_manager::InputManager::new();
    let mut ship = Ship::new();
    let mut projectiles = Vec::new();
    // let mut polys = polygon::gen_polys(polygon::NUMBER_OF_POLYGONS);

    input.register_key(KeyCode::A);
    input.register_key(KeyCode::Space);
    input.register_key(KeyCode::Up);
    input.register_key(KeyCode::Down);
    input.register_key(KeyCode::Left);
    input.register_key(KeyCode::Right);
    loop {
        input.update();
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        // shoot!
        if input.debounced_is_key_down(KeyCode::Space) {
            projectiles.push(ship.shoot());
        }

        // move
        if is_key_down(KeyCode::Right) {
            ship.rotate(5.0);
        }
        if is_key_down(KeyCode::Left) {
            ship.rotate(-5.0);
        }

        // rendering
        // TODO: reset game
        // TODO: render(&mut enemies);
        render(&mut projectiles);
        render_one(&mut ship);
        next_frame().await;
    }
}
