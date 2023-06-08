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
    #[structof(nested)]
    pub body: Body,
    pub damage: Hp,
}

#[derive(StructOf, Debug)]
pub struct Actor {
    #[structof(nested)]
    pub body: Body,
    pub health: Health,
    // #[structof(nested)] // TODO: optional nesting
    pub gun: Option<Gun>,
    pub stats: Stats,
    pub controller: Controller,
    pub ai: Option<ActorAI>,
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

#[derive(Debug, Clone)]
pub enum ActorAI {
    Crawler,
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
    pub fn new(pos: vec2<Coord>, target: vec2<Coord>, config: ProjectileConfig) -> Self {
        Self {
            body: Body::new(pos, config.shape)
                .with_velocity((target - pos).normalize_or_zero() * config.speed),
            damage: config.damage,
        }
    }
}

impl Actor {
    pub fn new(body: Body, hp: Hp, acceleration: Coord, stats: Stats) -> Self {
        Self {
            body,
            health: Health::new(hp),
            gun: None,
            stats,
            controller: Controller {
                target_velocity: vec2::ZERO,
                acceleration,
            },
            ai: None,
        }
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
}
