use macroquad::prelude::{
    get_frame_time, is_key_down, KeyCode,
};
use std::time::{Duration};
use std::collections::HashMap;

pub struct InputManager {
    keys_pressed: HashMap<KeyCode, (bool, i32)>,
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


impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            keys_pressed: HashMap::new(),
        }
    }
    // debounce_time in ms
    fn debounced_is_key_down(
        &mut self,
        key: KeyCode,
        debounce_time: Duration,
        cur_frame: i32,
    ) -> bool {
        let when_started_being_down = self.keys_pressed.get(&key).copied().unwrap_or((false, 0));

        if !is_key_down(key) {
            self.keys_pressed.insert(key, (false, 0)); // key is no longer or not pressed, reset
        } else if is_key_down(key) && when_started_being_down.1 == 0 {
            self.keys_pressed.insert(key, (true, cur_frame)); // key just got pressed
        } else {
            self.keys_pressed.insert(key, (true, when_started_being_down.1)); // key is still being pressed
        }

        let first_down_frame = self.keys_pressed[&key].1;
        if self.keys_pressed[&key].0 && time_since(first_down_frame, cur_frame) > debounce_time {
            self.keys_pressed.insert(key, (false, 0));
            return true;
        }
        return false;
    }

    pub fn is_down(&mut self, key: KeyCode, cur_frame: i32) -> bool {
        return self.debounced_is_key_down(key, Duration::from_millis(50), cur_frame);
    }
}
