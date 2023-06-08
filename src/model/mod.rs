mod collider;
mod logic;
mod shape;

pub use self::{collider::*, shape::*};

use crate::util::{Mat3RealConversions, RealConversions, Vec2RealConversions};

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;

#[derive(Debug)]
pub struct Player {
    pub body: Id,
    pub player_direction: vec2<Coord>,
    pub player_speed: vec2<Coord>,
    pub player_acceleration: vec2<Coord>,
    pub target_velocity: vec2<Coord>,

    pub out_of_view: bool,
}

#[derive(StructOf, Debug)]
pub struct Body {
    #[structof(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
}

pub struct Model {
    pub camera: Camera2d,
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
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 50.0,
            },
            player: Player {
                body: player_body,
                player_direction: vec2::ZERO,
                player_speed: vec2(45, 45).as_r32(),
                player_acceleration: vec2(3, 3).as_r32(),
                target_velocity: vec2::ZERO,
                out_of_view: false,
            },
            bodies,
        }
    }
}
