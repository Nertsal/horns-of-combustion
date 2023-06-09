mod action;
mod camera;
mod components;
mod health;
mod logic;
mod player;
mod weapons;

pub use self::{action::*, camera::*, components::*, health::*, player::*, weapons::*};

use crate::{
    assets::config::*,
    util::{RealConversions, Vec2RealConversions},
};

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;
pub type Lifetime = Health;

pub struct Model {
    pub config: Config,
    pub camera: Camera,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub gasoline: StructOf<Arena<Gasoline>>,
    pub fire: StructOf<Arena<Fire>>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let mut actors = StructOf::new();
        let mut model = Self {
            camera: Camera::new(config.camera.fov),
            player: Player::init(config.player, &mut actors),
            actors,
            projectiles: StructOf::new(),
            gasoline: StructOf::new(),
            fire: StructOf::new(),
            config,
        };
        model.init();
        model
    }

    fn init(&mut self) {
        self.actors.insert(
            Actor::new(
                Body::new(vec2(15, 0).as_r32(), Shape::Circle { radius: r32(1.0) }),
                r32(50.0),
                r32(1.0),
                Fraction::Enemy,
                Stats {
                    move_speed: r32(10.0),
                },
            )
            .with_ai(ActorAI::Crawler),
        );
    }
}
