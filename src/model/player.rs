use super::*;

#[derive(Debug)]
pub struct Player {
    pub body: Body,
    pub player_direction: vec2<Coord>,
    pub target_velocity: vec2<Coord>,
    pub out_of_view: bool,
    pub gun: Gun,
}

impl Player {
    pub fn new(config: PlayerConfig) -> Self {
        Self {
            body: Body::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
            player_direction: vec2::ZERO,
            target_velocity: vec2::ZERO,
            out_of_view: false,
            gun: Gun::new(config.gun),
        }
    }
}
