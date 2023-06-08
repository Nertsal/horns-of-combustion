use super::*;

#[derive(Debug)]
pub struct Player {
    pub actor: Actor,
    pub player_direction: vec2<Coord>,
    pub target_velocity: vec2<Coord>,
    pub out_of_view: bool,
}

impl Player {
    pub fn new(config: PlayerConfig) -> Self {
        Self {
            actor: Actor::new(
                Body::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
                config.hp,
                config.gun,
            ),
            player_direction: vec2::ZERO,
            target_velocity: vec2::ZERO,
            out_of_view: false,
        }
    }
}
