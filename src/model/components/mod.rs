mod collider;

pub use self::collider::*;

use super::*;

#[derive(StructOf, Debug)]
pub struct Body {
    #[structof(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

#[derive(StructOf, Debug)]
pub struct Projectile {
    pub fraction: Fraction,
    #[structof(nested)]
    pub body: Body,
    pub damage: Hp,
    pub target_pos: Option<vec2<Coord>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Player,
    Enemy,
}

#[derive(StructOf, Debug)]
pub struct Actor {
    pub fraction: Fraction,
    #[structof(nested)]
    pub body: Body,
    pub health: Health,
    // #[structof(nested)] // TODO: optional nesting
    pub gun: Option<Gun>,
    pub stats: Stats,
    pub controller: Controller,
    pub ai: Option<ActorAI>,
    pub stops_barrel: bool,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub move_speed: Coord,
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub target_velocity: vec2<Coord>,
    pub acceleration: Coord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ActorAI {
    Crawler,
}

#[derive(StructOf, Debug)]
pub struct Gasoline {
    pub collider: Collider,
    pub lifetime: Lifetime,
}

#[derive(StructOf, Debug)]
pub struct Fire {
    pub collider: Collider,
    pub lifetime: Lifetime,
}

impl Body {
    pub fn new(pos: vec2<Coord>, shape: Shape) -> Self {
        Self {
            collider: Collider::new(pos, shape),
            velocity: vec2::ZERO,
        }
    }

    pub fn with_velocity(self, velocity: vec2<Coord>) -> Self {
        Self { velocity, ..self }
    }
}

impl Projectile {
    pub fn new(
        pos: vec2<Coord>,
        target: vec2<Coord>,
        fraction: Fraction,
        config: ProjectileConfig,
    ) -> Self {
        Self {
            fraction,
            body: Body::new(pos, config.shape)
                .with_velocity((target - pos).normalize_or_zero() * config.speed),
            damage: config.damage,
            target_pos: None,
        }
    }

    pub fn with_target(self, target_pos: vec2<Coord>) -> Self {
        Self {
            target_pos: Some(target_pos),
            ..self
        }
    }
}

impl Actor {
    pub fn new(body: Body, hp: Hp, acceleration: Coord, fraction: Fraction, stats: Stats) -> Self {
        Self {
            fraction,
            body,
            health: Health::new(hp),
            gun: None,
            stats,
            controller: Controller {
                target_velocity: vec2::ZERO,
                acceleration,
            },
            ai: None,
            stops_barrel: false,
        }
    }

    pub fn new_enemy(pos: vec2<Coord>, config: EnemyConfig) -> Self {
        Self::new(
            Body::new(pos, config.shape),
            config.hp,
            config.acceleration,
            Fraction::Enemy,
            Stats {
                move_speed: config.speed,
            },
        )
        .with_ai(config.ai)
        .stop_barrel(config.stops_barrel)
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
