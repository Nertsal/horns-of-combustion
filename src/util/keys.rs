use crate::assets::controls::Key;

use geng::Window;

pub fn is_key_pressed(window: &Window, keys: &[Key]) -> bool {
    keys.iter().any(|key| key.is_pressed(window))
}

pub fn is_event_down(event: &geng::Event, keys: &[Key]) -> bool {
    keys.iter().any(|key| key.is_event_down(event))
}

pub fn is_event_up(event: &geng::Event, keys: &[Key]) -> bool {
    keys.iter().any(|key| key.is_event_up(event))
}
