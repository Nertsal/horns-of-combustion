use super::*;

#[derive(Debug)]
pub struct Player {
    pub actor: Id,
    pub input: PlayerInput,
    pub out_of_view: bool,
    pub state: PlayerState,
    pub gasoline: Bounded<R32>,
}

#[derive(Debug)]
pub struct PlayerInput {
    pub aim_at: Position,
    pub direction: vec2<Coord>,
    pub drip_gas: bool,
}

#[derive(Debug, Clone)]
pub enum PlayerState {
    Human,
    Barrel { last_gas: Position },
}

impl Player {
    pub fn new(actor: Id, world_size: vec2<Coord>) -> Self {
        Self {
            actor,
            input: PlayerInput {
                aim_at: Position::zero(world_size),
                direction: vec2::ZERO,
                drip_gas: false,
            },
            out_of_view: false,
            state: PlayerState::Human,
            gasoline: Bounded::new(R32::ZERO, R32::ZERO..=r32(100.0)),
        }
    }

    pub fn init(
        config: PlayerConfig,
        world_size: vec2<Coord>,
        actors: &mut StructOf<Arena<Actor>>,
    ) -> Self {
        let actor = actors.insert(
            Actor::new(
                Body::new(Position::zero(world_size), config.human_state.body),
                config.hp,
                config.acceleration,
                Fraction::Player,
                config.human_state.stats,
                ActorKind::Player,
            )
            .with_gun(config.gun),
        );
        Self::new(actor, world_size)
    }
}
