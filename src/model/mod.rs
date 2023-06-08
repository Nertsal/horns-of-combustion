mod collider;
mod logic;
mod shape;

pub use self::{collider::*, shape::*};

use crate::util::{Mat3RealConversions, RealConversions, Vec2RealConversions};

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;

#[derive(Debug)]
pub struct Player {
    pub body: Id,
}

#[derive(StructOf, Debug)]
pub struct Body {
    #[structof(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

pub struct Model {
    pub player: Player,
    pub bodies: StructOf<Arena<Body>>,
}

impl Model {
    pub fn new() -> Self {
        let mut bodies = StructOf::<Arena<Body>>::new();
        let player_body = bodies.insert(Body {
            collider: Collider::new(vec2::ZERO, Shape::Circle { radius: r32(1.0) }),
            velocity: vec2::ZERO,
        });

        Self {
            player: Player { body: player_body },
            bodies,
        }
    }
}
