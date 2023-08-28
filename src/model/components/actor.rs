use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Player,
    Enemy,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stats {
    pub contact_damage: Hp,
    pub move_speed: Coord,
    pub vulnerability: VulnerabilityStats,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(default)]
pub struct VulnerabilityStats {
    /// Resistance from contact damage and bullets.
    pub physical: R32,
    /// Resistance from contact damage and bullets.
    pub projectile: R32,
    /// Resistance from fire damage.
    pub fire: R32,
    /// Resistance from explosions.
    pub explosive: R32,
}

impl Default for VulnerabilityStats {
    fn default() -> Self {
        Self {
            physical: R32::ONE,
            projectile: R32::ONE,
            fire: R32::ONE,
            explosive: R32::ONE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub target_velocity: vec2<Coord>,
    pub acceleration: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorAI {
    Crawler,
    Ranger { preferred_distance: Coord },
    BossFoot { position: Position },
    BossBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorKind {
    Player,
    EnemyClown,
    EnemyDeathStar,
    EnemyDice,
    EnemyHuge,
    BossFoot { leg_offset: vec2<Coord> },
    BossBody,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OnFire {
    pub duration: Time,
    pub damage_per_second: Hp,
}

#[derive(SplitFields, Debug)]
pub struct Actor {
    pub fraction: Fraction,
    #[split(nested)]
    pub body: Body,
    pub health: Health,
    // #[split(nested)] // TODO: optional nesting
    pub gun: Option<Gun>,
    pub stats: Stats,
    pub controller: Controller,
    pub ai: Option<ActorAI>,
    pub kind: ActorKind,
    pub stops_barrel: bool,
    pub stunned: Option<Time>,
    pub on_fire: Option<OnFire>,
}

impl Actor {
    pub fn new(
        body: Body,
        hp: Hp,
        acceleration: Coord,
        fraction: Fraction,
        stats: Stats,
        kind: ActorKind,
    ) -> Self {
        Self {
            fraction,
            body,
            health: Health::new_max(hp),
            gun: None,
            stats,
            controller: Controller {
                target_velocity: vec2::ZERO,
                acceleration,
            },
            ai: None,
            kind,
            stops_barrel: false,
            stunned: None,
            on_fire: None,
        }
    }

    pub fn new_enemy(pos: Position, config: EnemyConfig) -> Self {
        let mut enemy = Self::new(
            Body::new(pos, config.body),
            config.hp,
            config.acceleration,
            Fraction::Enemy,
            config.stats,
            config.kind,
        )
        .with_ai(config.ai)
        .stop_barrel(config.stops_barrel);
        if let Some(gun) = config.gun {
            enemy = enemy.with_gun(gun);
        }
        enemy
    }

    pub fn with_gun(self, gun: GunConfig) -> Self {
        Self {
            gun: Some(Gun::new(gun)),
            ..self
        }
    }

    pub fn with_ai(self, ai: ActorAI) -> Self {
        Self {
            ai: Some(ai),
            ..self
        }
    }

    pub fn stop_barrel(self, stops_barrel: bool) -> Self {
        Self {
            stops_barrel,
            ..self
        }
    }
}
