use super::*;

#[derive(Debug)]
pub struct Player {
    pub actor: Id,
    pub player_direction: vec2<Coord>,
    pub target_velocity: vec2<Coord>,
    pub out_of_view: bool,
}

impl Player {
    pub fn new(actor: Id) -> Self {
        Self {
            actor,
            player_direction: vec2::ZERO,
            target_velocity: vec2::ZERO,
            out_of_view: false,
        }
    }

    pub fn init(config: PlayerConfig, actors: &mut StructOf<Arena<Actor>>) -> Self {
        let actor = actors.insert(
            Actor::new(
                Body::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
                config.hp,
                config.acceleration,
                Stats {
                    move_speed: config.speed,
                },
            )
            .with_gun(config.gun),
        );
        Self::new(actor)
    }
}
