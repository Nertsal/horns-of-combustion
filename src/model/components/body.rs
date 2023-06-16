use super::*;

#[derive(SplitFields, Debug)]
pub struct Body {
    #[split(nested)]
        pub collider: Collider,
    pub velocity: vec2<Coord>,
}

impl Body {
    pub fn new(pos: Position, shape: Shape) -> Self {
        Self {
            collider: Collider::new(pos, shape),
            velocity: vec2::ZERO,
        }
    }

    pub fn with_velocity(self, velocity: vec2<Coord>) -> Self {
        Self { velocity, ..self }
    }
}
