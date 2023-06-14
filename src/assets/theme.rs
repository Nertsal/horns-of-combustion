use super::*;

use crate::model::Color;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    pub palette: Palette,
    pub level: LevelTheme,
    pub background: Color,
    pub collider_color: Color,
    pub spawn_circle_color: Color,
    pub outline_color: Color,
    pub gasoline: Color,
    pub fire: Color,
    pub fire_particles: Color,
    pub health_bg: Color,
    pub health_fg: Color,
    pub pickups: PickUpsTheme,
}

pub type Palette = HashMap<String, Color>;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PickUpsTheme {
    pub heal: Color,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelTheme {
    pub background: Vec<String>,
    pub foreground: Vec<String>,
}

impl Theme {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }

    pub fn get_palette(&self, names: &[String]) -> Vec<Color> {
        names
            .iter()
            .map(|name| {
                *self.palette.get(name).unwrap_or_else(|| {
                    panic!(
                        "Color {} not found in the palette: {:?}",
                        name, self.palette
                    )
                })
            })
            .collect()
    }
}
