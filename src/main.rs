mod input_manager;
mod utils;

mod enemy;
mod objects;
mod projectile;
mod ship;

use enemy::Enemy;
use ship::Ship;

use macroquad::prelude::{is_key_down, is_key_pressed, next_frame, Conf, KeyCode};
use std::collections::HashMap;

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
    input.register_key(KeyCode::A);
    input.register_key(KeyCode::Space);
    input.register_key(KeyCode::Up);
    input.register_key(KeyCode::Down);
    input.register_key(KeyCode::Left);
    input.register_key(KeyCode::Right);

    let mut ship = Ship::new();
    let mut projectiles = Vec::new();
    let mut enemies = Vec::new();

    let mut frame_number = 0;
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

        // new enemy every 100 frames
        // TODO: enemies start slowly and come faster the more points you have (needs points)
        if frame_number % 50 == 0 {
            enemies.push(Enemy::new(ship.object.position));
        }

        // rendering
        // TODO: reset game
        render_one(&mut ship);
        render(&mut projectiles);
        render(&mut enemies);
        next_frame().await;
        frame_number += 1;
    }
}
