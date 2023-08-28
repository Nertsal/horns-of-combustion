use super::*;

use crate::model::{
    ActorAI, ActorKind, BlockKind, Coord, Hp, OnFire, ProjectileAI, ProjectileKind, Shape,
    ShotPattern, Stats, Time, VulnerabilityStats,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// Size of the world torus.
    pub world_size: vec2<Coord>,
    pub explosions_affect_projectiles: bool,
    pub death_explosion: Option<ExplosionConfig>,
    pub death_drop_heal_chance: R32,
    pub pickups: PickupConfig,
    pub player: PlayerConfig,
    pub camera: CameraConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LevelConfig {
    pub foreground: ProcGenConfig,
    pub background: ProcGenConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PickupConfig {
    pub size: Coord,
    pub heal_amount: Hp,
    pub attract_radius: Coord,
    pub attract_strength: Coord,
    pub max_speed: Coord,
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
    #[serde(default)]
    pub vulnerability: VulnerabilityStats,
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
    /// Increase in speed from a barrel dash.
    pub dash_burst: Coord,
    pub stats: Stats,
    pub acceleration: Coord,
    pub hp: Hp,
    pub gun: GunConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HumanStateConfig {
    pub body: BodyConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BarrelStateConfig {
    /// Max possible speed.
    pub speed: Coord,
    pub dash_speed: Coord,
    pub dash_explosion: ExplosionConfig,
    pub steering: R32,
    pub runover_damage: Hp,
    pub runover_damage_scale: Hp,
    pub self_explosion_strength: Coord,
    pub body: BodyConfig,
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
    #[serde(default)]
    pub ignite_gasoline: bool,
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
    pub body: BodyConfig,
    #[serde(default)]
    pub ai: ProjectileAI,
    #[serde(default)]
    pub kind: ProjectileKind,
    pub knockback: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnemyConfig {
    pub body: BodyConfig,
    pub stats: Stats,
    pub acceleration: Coord,
    pub hp: Hp,
    pub ai: ActorAI,
    pub kind: ActorKind,
    pub gun: Option<GunConfig>,
    #[serde(default)]
    pub stops_barrel: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BodyConfig {
    pub shape: Shape,
    #[serde(default = "BodyConfig::default_mass")]
    pub mass: R32,
}

impl BodyConfig {
    fn default_mass() -> R32 {
        R32::ONE
    }
}

impl Config {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        crate::util::load_file(path).await
    }

    pub async fn load_enemies(
        path: impl AsRef<std::path::Path>,
    ) -> anyhow::Result<HashMap<String, EnemyConfig>> {
        let path = path.as_ref();
        log::debug!("Loading folder {:?}", path);

        let list: Vec<String> = crate::util::load_file(&path.join("_list.ron")).await?;

        let mut enemies = HashMap::new();
        for name in list {
            let enemy: EnemyConfig =
                crate::util::load_file(&path.join(&name).with_extension("ron")).await?;
            enemies.insert(name, enemy);
        }
        Ok(enemies)
    }
}
