use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct SliderInputState {
    pub key_repeat_timer: Timer,
    pub held_key: Option<KeyCode>,
}

impl Default for SliderInputState {
    fn default() -> Self {
        Self {
            key_repeat_timer: Timer::new(Duration::from_millis(300), TimerMode::Once),
            held_key: None,
        }
    }
}