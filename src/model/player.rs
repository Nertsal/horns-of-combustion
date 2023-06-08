use super::*;

#[derive(Debug)]
pub struct Player {
    pub body: Id,
    pub player_direction: vec2<Coord>,
    pub target_velocity: vec2<Coord>,
    pub out_of_view: bool,
}

impl Player {
    pub fn new(body: Id) -> Self {
        Self {
            body,
            player_direction: vec2::ZERO,
            target_velocity: vec2::ZERO,
            out_of_view: false,
        }
    }

    pub fn init(bodies: &mut StructOf<Arena<Body>>) -> Self {
        let player_body = bodies.insert(Body {
            collider: Collider::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
            velocity: vec2::ZERO,
        });
        Self::new(player_body)
    }
}
