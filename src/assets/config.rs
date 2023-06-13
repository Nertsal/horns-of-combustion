use super::*;

use crate::model::{
    ActorAI, BlockKind, Coord, Hp, OnFire, ProjectileAI, ProjectileKind, Shape, ShotPattern, Time,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Size of the world torus.
    pub world_size: vec2<Coord>,
    pub death_explosion: Option<ExplosionConfig>,
    pub player: PlayerConfig,
    pub camera: CameraConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelConfig {
    pub foreground: ProcGenConfig,
    pub background: ProcGenConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcGenConfig {
    /// Min space between the blocks.
    pub spacing: Coord,
    /// The total number of blocks to spawn.
    pub blocks_number: usize,
    /// Variants of blocks to spawn.
    pub blocks: Vec<BlockConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockConfig {
    pub health: Option<Hp>,
    #[serde(default = "default_weight")]
    pub weight: R32,
    #[serde(default = "default_block")]
    pub kind: BlockKind,
    pub shape: Shape,
    pub explosion: Option<ExplosionConfig>,
}

fn default_weight() -> R32 {
    R32::ONE
}

fn default_block() -> BlockKind {
    BlockKind::Obstacle
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    pub fire_immune: bool,
    /// Increase in speed from a barrel dash.
    pub dash_burst: Coord,
    /// Damage to deal to enemies upon contact.
    pub contact_damage: Hp,
    pub speed: Coord,
    pub acceleration: Coord,
    pub hp: Hp,
    pub gun: GunConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanStateConfig {
    pub shape: Shape,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BarrelStateConfig {
    /// Max possible speed.
    pub speed: Coord,
    pub steering: R32,
    pub runover_damage: Hp,
    pub runover_damage_scale: Hp,
    pub self_explosion_strength: Coord,
    pub shape: Shape,
    pub gasoline: GasolineConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GasolineConfig {
    /// Whether gasoline dripping can be controlled (turned on/off).
    pub can_control: bool,
    pub cost: R32,
    pub lifetime: Time,
    pub distance_period: Coord,
    pub ignite_timer: Time,
    pub fire_radius: Coord,
    pub explosion: ExplosionConfig,
    pub shape: Shape,
    pub fire: FireConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExplosionConfig {
    pub radius: Coord,
    pub knockback: Coord,
    pub damage: Hp,
    pub ignite: Option<OnFire>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FireConfig {
    pub duration: Time,
    pub damage_per_second: Hp,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GunConfig {
    /// Delay between shots.
    pub shot_delay: Time,
    pub shot: ShotConfig,
    pub recoil: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShotConfig {
    #[serde(default)]
    pub pattern: ShotPattern,
    pub projectile: ProjectileConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectileConfig {
    pub lifetime: Time,
    pub speed: Coord,
    pub damage: Hp,
    pub shape: Shape,
    #[serde(default)]
    pub ai: ProjectileAI,
    #[serde(default)]
    pub kind: ProjectileKind,
    pub knockback: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnemyConfig {
    pub contact_damage: Hp,
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

    pub async fn load_enemies(
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<HashMap<String, EnemyConfig>> {
        let path = path.as_ref();
        log::debug!("Loading folder {:?}", path);
        let list_path = path.join("_list.ron");
        let list: Vec<String> = file::load_detect(&list_path)
            .await
            .context(format!("when loading {:?}", list_path))?;

        let mut enemies = HashMap::new();
        for name in list {
            let path = path.join(&name).with_extension("ron");
            let enemy: EnemyConfig = file::load_detect(&path)
                .await
                .context(format!("when loading {:?}", path))?;
            enemies.insert(name, enemy);
        }
        Ok(enemies)
    }
}
