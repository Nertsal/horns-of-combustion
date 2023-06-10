use super::*;

use crate::model::{ActorAI, Coord, Hp, ProjectileAI, Shape, ShotPattern, Time};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub player: PlayerConfig,
    pub camera: CameraConfig,
    pub enemies: Vec<EnemyConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CameraConfig {
    pub fov: Coord,
    pub speed: Coord,
    /// Radius in which the camera allows the target to move without affecting the camera.
    pub dead_zone: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerConfig {
    pub human_state: HumanStateConfig,
    pub barrel_state: BarrelStateConfig,
    /// Increase in speed from a barrel dash.
    pub dash_burst: Coord,
    pub speed: Coord,
    pub acceleration: Coord,
    pub hp: Hp,
    pub gun: GunConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct HumanStateConfig {
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct BarrelStateConfig {
    /// Max possible speed.
    pub speed: Coord,
    pub steering: R32,
    pub shape: Shape,
    pub gasoline: GasolineConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct GasolineConfig {
    pub lifetime: Time,
    pub distance_period: Coord,
    pub ignite_timer: Time,
    pub shape: Shape,
    pub fire: FireConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct FireConfig {
    pub duration: Time,
    pub damage_per_second: Hp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GunConfig {
    /// Delay between shots.
    pub shot_delay: Time,
    pub shot: ShotConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShotConfig {
    #[serde(default)]
    pub pattern: ShotPattern,
    pub projectile: ProjectileConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectileConfig {
    pub speed: Coord,
    pub damage: Hp,
    pub shape: Shape,
    #[serde(default)]
    pub ai: ProjectileAI,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnemyConfig {
    pub shape: Shape,
    pub speed: Coord,
    pub acceleration: Coord,
    pub hp: Hp,
    pub ai: ActorAI,
    pub gun: Option<GunConfig>,
    #[serde(default)]
    pub stops_barrel: bool,
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }
}
