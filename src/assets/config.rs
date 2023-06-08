use super::*;

use crate::model::{Coord, Shape, Time};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Config {
    pub player: PlayerConfig,
    pub camera: CameraConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct PlayerConfig {
    pub speed: Coord,
    pub acceleration: Coord,
    pub gun: GunConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CameraConfig {
    pub fov: Coord,
    pub speed: Coord,
    /// Radius in which the camera allows the target to move without affecting the camera.
    pub dead_zone: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GunConfig {
    /// Delay between shots.
    pub shot_delay: Time,
    pub projectile: ProjectileConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ProjectileConfig {
    pub speed: Coord,
    pub shape: Shape,
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let config = file::load_detect(path).await?;
        Ok(config)
    }
}
