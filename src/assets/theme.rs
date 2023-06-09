use super::*;

use crate::model::Color;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Theme {
    pub background: Color,
    pub player: Color,
    pub projectile: Color,
    pub gasoline: Color,
    pub fire: Color,
    pub enemies: EnemiesTheme,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct EnemiesTheme {
    pub crawler: Color,
}

impl Theme {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let theme = file::load_detect(path).await?;
        Ok(theme)
    }
}
