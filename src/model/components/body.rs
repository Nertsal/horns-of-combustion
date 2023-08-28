use super::*;

#[derive(SplitFields, Debug)]
pub struct Body {
    #[split(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub mass: R32,
}

impl Body {
    pub fn new(pos: Position, config: BodyConfig) -> Self {
        Self {
            collider: Collider::new(pos, config.shape),
            velocity: vec2::ZERO,
            mass: config.mass,
        }
    }

    pub fn with_velocity(self, velocity: vec2<Coord>) -> Self {
        Self { velocity, ..self }
    }
}
