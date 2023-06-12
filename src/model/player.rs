use super::*;

#[derive(Debug)]
pub struct Player {
    pub actor: Id,
    pub input: PlayerInput,
    pub out_of_view: bool,
    pub state: PlayerState,
    pub gasoline: Health,
}

#[derive(Debug)]
pub struct PlayerInput {
    pub aim_at: vec2<Coord>,
    pub direction: vec2<Coord>,
    pub drip_gas: bool,
}

#[derive(Debug, Clone)]
pub enum PlayerState {
    Human,
    Barrel { last_gas: Position },
}

impl Player {
    pub fn new(actor: Id) -> Self {
        Self {
            actor,
            input: PlayerInput {
                aim_at: vec2::ZERO,
                direction: vec2::ZERO,
                drip_gas: false,
            },
            out_of_view: false,
            state: PlayerState::Human,
            gasoline: Health {
                hp: r32(0.0),
                max_hp: r32(100.0),
            },
        }
    }

    pub fn init(config: PlayerConfig, actors: &mut StructOf<Arena<Actor>>) -> Self {
        let actor = actors.insert(
            Actor::new(
                Body::new(Position::ZERO, Shape::Circle { radius: r32(1.0) }),
                config.hp,
                config.acceleration,
                Fraction::Player,
                Stats {
                    fire_immune: true,
                    contact_damage: config.contact_damage,
                    move_speed: config.speed,
                },
            )
            .with_gun(config.gun),
        );
        Self::new(actor)
    }
}
