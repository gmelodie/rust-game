use instant;
use macroquad::prelude::{is_key_down, KeyCode};
use std::collections::HashMap;
use std::time::Duration;

struct KeyState {
    first_frame: bool,
    last_repeated: instant::Instant,
}

pub struct InputManager {
    keys_pressed: HashMap<KeyCode, KeyState>,
}

const KEY_DEBOUNCE_TIME: f32 = 0.1;

impl Default for KeyState {
    fn default() -> Self {
        Self {
            first_frame: true,
            last_repeated: instant::Instant::now(),
        }
    }
}

impl InputManager {
    pub fn new() -> InputManager {
        InputManager {
            keys_pressed: HashMap::new(),
        }
    }
    // debounce_time in ms
    pub fn debounced_is_key_down(&mut self, key: KeyCode) -> bool {
        // return false if key is down
        if !is_key_down(key) {
            return false;
        }

        let mut key_state = self.keys_pressed.get_mut(&key).unwrap();

        // if key just started being pressed
        // return immediately
        if key_state.first_frame {
            key_state.first_frame = false;
            return true;
        }

        // if the last time we repeated was
        // long enough ago, return true and
        // update last_repeated
        if key_state.last_repeated.elapsed() > Duration::from_secs_f32(KEY_DEBOUNCE_TIME) {
            key_state.last_repeated = instant::Instant::now();
            return true;
        }

        return false;
    }

    pub fn update(&mut self) {
        for (key, state) in self.keys_pressed.iter_mut() {
            if !is_key_down(*key) {
                // reset first frame
                state.first_frame = true;
                state.last_repeated = instant::Instant::now();
            }
        }
    }

    pub fn register_key(&mut self, key: KeyCode) {
        self.keys_pressed.insert(
            key,
            KeyState {
                first_frame: false,
                last_repeated: instant::Instant::now(),
            },
        );
    }
}
