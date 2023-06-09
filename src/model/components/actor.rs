use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Player,
    Enemy,
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
    pub stunned: Option<Time>,
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
            stunned: None,
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
