use super::*;

/// A position on a torus.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position(vec2<Coord>);

impl Position {
    pub const ZERO: Self = Self(vec2::ZERO);

    pub fn from_world(mut pos: vec2<Coord>, world_size: vec2<Coord>) -> Self {
        // Normalize position
        while pos.y < Coord::ZERO {
            pos.y += world_size.y;
        }
        while pos.y > world_size.y {
            pos.y -= world_size.y;
        }
        while pos.x < Coord::ZERO {
            pos.x += world_size.x;
            // pos.y = world_size.y - pos.y;
        }
        while pos.x > world_size.x {
            pos.x -= world_size.x;
            // pos.y = world_size.y - pos.y;
        }

        Self(pos)
    }

    pub fn to_world(self) -> vec2<Coord> {
        self.0
    }

    pub fn to_world_f32(self) -> vec2<f32> {
        self.0.map(Coord::as_f32)
    }

    pub fn shift(&mut self, direction: vec2<Coord>, world_size: vec2<Coord>) {
        *self = self.shifted(direction, world_size);
    }

    pub fn shifted(&self, direction: vec2<Coord>, world_size: vec2<Coord>) -> Self {
        Self::from_world(self.to_world() + direction, world_size)
    }

    pub fn direction(&self, towards: Self, world_size: vec2<Coord>) -> vec2<Coord> {
        let mut delta = towards.to_world() - self.to_world();

        // Normalize delta
        if delta.x.abs() > world_size.x / Coord::new(2.0) {
            let signum = delta.x.signum();
            delta.x -= world_size.x * signum;
        }
        if delta.y.abs() > world_size.y / Coord::new(2.0) {
            let signum = delta.y.signum();
            delta.y -= world_size.y * signum;
        }

        delta
    }

    #[allow(dead_code)]
    pub fn distance(&self, other: Self, world_size: vec2<Coord>) -> Coord {
        self.direction(other, world_size).len()
    }

    #[allow(dead_code)]
    pub fn random(rng: &mut impl Rng, world_size: vec2<Coord>) -> Self {
        Self::from_world(
            vec2(
                rng.gen_range(Coord::ZERO..=world_size.x),
                rng.gen_range(Coord::ZERO..=world_size.y),
            ),
            world_size,
        )
    }
}
