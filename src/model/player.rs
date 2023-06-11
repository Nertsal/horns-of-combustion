use super::*;

#[derive(Debug)]
pub struct Player {
    pub actor: Id,
    pub aim_at: vec2<Coord>,
    pub input_direction: vec2<Coord>,
    pub out_of_view: bool,
    pub state: PlayerState,
}

#[derive(Debug, Clone)]
pub enum PlayerState {
    Human,
    Barrel { last_gas: vec2<Coord> },
}

impl Player {
    pub fn new(actor: Id) -> Self {
        Self {
            actor,
            aim_at: vec2::ZERO,
            input_direction: vec2::ZERO,
            out_of_view: false,
            state: PlayerState::Human,
        }
    }

    pub fn init(config: PlayerConfig, actors: &mut StructOf<Arena<Actor>>) -> Self {
        let actor = actors.insert(
            Actor::new(
                Body::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
                config.hp,
                config.acceleration,
                Fraction::Player,
                Stats {
                    move_speed: config.speed,
                },
            )
            .with_gun(config.gun),
        );
        Self::new(actor)
    }
}
