mod input_manager;
mod utils;

mod enemy;
mod objects;
mod projectile;
mod ship;

use enemy::Enemy;
use ship::Ship;
use utils::draw_header;

use macroquad::prelude::{
    draw_text, is_key_down, is_key_pressed, next_frame, screen_height, screen_width, Conf, KeyCode,
    WHITE,
};

const MAX_SPEED: f32 = 3.0;

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

#[macroquad::main("game-rust")]
async fn main() {
    let mut input = input_manager::InputManager::new();

    input.register_key(KeyCode::W);
    input.register_key(KeyCode::A);
    input.register_key(KeyCode::S);
    input.register_key(KeyCode::D);
    input.register_key(KeyCode::Space);
    input.register_key(KeyCode::Up);
    input.register_key(KeyCode::Down);
    input.register_key(KeyCode::Left);
    input.register_key(KeyCode::Right);

    let mut ship = Ship::new();
    let mut projectiles = Vec::new();
    let mut enemies = Vec::new();

    let mut points = 0;

    let mut frame_number = 0;
    loop {
        draw_header(points, ship.lives);
        input.update();
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        // shoot!
        if input.debounced_is_key_down(KeyCode::Space) {
            projectiles.push(ship.shoot());
        }

        // move
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            ship.rotate(3.0);
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            ship.rotate(-3.0);
        }
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            if ship.object.speed < MAX_SPEED {
                ship.object.speed += 0.1;
            }
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            ship.object.speed -= 0.1;
            if ship.object.speed < 0.0 {
                ship.object.speed = 0.0;
            }
        }

        // new enemy every 100 frames
        // TODO: enemies start slowly and come faster the more points you have (needs points)
        if frame_number % 20 == 0 {
            enemies.push(Enemy::new(ship.object.position, points));
        }

        render_one(&mut ship);
        render(&mut projectiles);
        render(&mut enemies);

        // check collisions of enemies and projectiles
        for enemy in enemies.iter_mut() {
            for projectile in projectiles.iter_mut() {
                if enemy.collided(&projectile) {
                    enemy.dead = true;
                    projectile.dead = true;
                    points += 1;
                }
            }
        }

        // check collisions of enemies and ship
        for enemy in enemies.iter_mut() {
            if ship.collided(enemy) {
                ship.lives -= 1;
                enemy.dead = true;
            }
        }

        // retain every enemy and projectile
        // that is not dead
        enemies.retain(|e| e.dead == false);
        projectiles.retain(|p| p.dead == false);

        if ship.lives <= 0 {
            break;
        }
        next_frame().await;
        frame_number += 1;
    }

    loop {
        draw_header(points, ship.lives);
        input.update();
        if is_key_pressed(KeyCode::Q) {
            break;
        }

        // shoot!
        if input.debounced_is_key_down(KeyCode::Space) {
            projectiles.push(ship.shoot());
        }

        draw_text(
            "you lost",
            screen_height() / 2.0,
            screen_width() / 2.0,
            100.0,
            WHITE,
        );
        next_frame().await;
    }
}
