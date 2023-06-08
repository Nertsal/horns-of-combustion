use super::*;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub camera: CameraConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerConfig {
    pub speed: Coord,
    pub acceleration: Coord,
}

#[derive(Serialize, Deserialize)]
pub struct CameraConfig {
    pub fov: Coord,
    pub speed: Coord,
    /// Radius in which the camera allows the target to move without affecting the camera.
    pub dead_zone: Coord,
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let s = file::load_string(path).await?;
        let config = toml::de::from_str(&s)?;
        Ok(config)
    }
}
