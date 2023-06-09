use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Key(geng::Key),
    Mouse(geng::MouseButton),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Controls {
    pub left: Vec<Key>,
    pub right: Vec<Key>,
    pub up: Vec<Key>,
    pub down: Vec<Key>,
    pub shoot: Vec<Key>,
    pub transform: Vec<Key>,
}

impl Key {
    pub fn is_pressed(self, window: &geng::Window) -> bool {
        match self {
            Key::Key(key) => window.is_key_pressed(key),
            Key::Mouse(button) => window.is_button_pressed(button),
        }
    }

    pub fn is_event_down(self, event: &geng::Event) -> bool {
        match (&self, event) {
            (Key::Key(self_key), geng::Event::KeyDown { key }) => self_key == key,
            (Key::Mouse(self_button), geng::Event::MouseDown { button, .. }) => {
                self_button == button
            }
            _ => false,
        }
    }
}

impl Controls {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }
}
