use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fraction {
    Player,
    Enemy,
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub fire_immune: bool,
    pub contact_damage: Hp,
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
    Ranger { preferred_distance: Coord },
}

#[derive(Debug, Clone)]
pub struct OnFire {
    pub duration: Time,
    pub damage_per_second: Hp,
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
    pub on_fire: Option<OnFire>,
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
            on_fire: None,
        }
    }

    pub fn new_enemy(pos: Position, config: EnemyConfig) -> Self {
        let mut enemy = Self::new(
            Body::new(pos, config.shape),
            config.hp,
            config.acceleration,
            Fraction::Enemy,
            Stats {
                fire_immune: false,
                contact_damage: config.contact_damage,
                move_speed: config.speed,
            },
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
