use super::*;

/// A position on a torus.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Position {
    pos: vec2<Coord>,
    world_size: vec2<Coord>,
}

impl Position {
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

        Self { pos, world_size }
    }

    pub fn zero(world_size: vec2<Coord>) -> Self {
        Self::from_world(vec2::ZERO, world_size)
    }

    pub fn random(rng: &mut impl Rng, world_size: vec2<Coord>) -> Self {
        Self::from_world(
            vec2(
                rng.gen_range(Coord::ZERO..=world_size.x),
                rng.gen_range(Coord::ZERO..=world_size.y),
            ),
            world_size,
        )
    }

    pub fn to_world(self) -> vec2<Coord> {
        self.pos
    }

    pub fn to_world_f32(self) -> vec2<f32> {
        self.pos.map(Coord::as_f32)
    }

    pub fn world_size(self) -> vec2<Coord> {
        self.world_size
    }

    /// Returns a delta from zero to `self`.
    pub fn as_dir(self) -> vec2<Coord> {
        Self::zero(self.world_size).delta_to(self)
    }

    pub fn shift(&mut self, direction: vec2<Coord>) {
        *self = self.shifted(direction);
    }

    pub fn shifted(self, direction: vec2<Coord>) -> Self {
        Self::from_world(self.to_world() + direction, self.world_size)
    }

    pub fn delta_to(self, towards: Self) -> vec2<Coord> {
        self.assert_world_size(towards);

        let mut delta = towards.to_world() - self.to_world();

        // Normalize delta
        if delta.x.abs() > self.world_size.x / Coord::new(2.0) {
            let signum = delta.x.signum();
            delta.x -= self.world_size.x * signum;
        }
        if delta.y.abs() > self.world_size.y / Coord::new(2.0) {
            let signum = delta.y.signum();
            delta.y -= self.world_size.y * signum;
        }

        delta
    }

    pub fn distance(self, other: Self) -> Coord {
        self.delta_to(other).len()
    }

    /// Check whether the world sizes match, panic if not.
    fn assert_world_size(self, other: Self) {
        assert_eq!(
            self.world_size, other.world_size,
            "two positions are not from the same world"
        )
    }
}
