use super::*;

use crate::model::Color;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub player: Color,
    pub projectile: Color,
    pub gasoline: Color,
    pub fire: Color,
    pub health_bg: Color,
    pub health_fg: Color,
    pub enemies: EnemiesTheme,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct EnemiesTheme {
    pub crawler: Color,
    pub ranger: Color,
}

impl Theme {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }
}
