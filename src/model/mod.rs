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
    util::{Mat3RealConversions, RealConversions, Vec2RealConversions},
};

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;

pub struct Model {
    pub config: Config,
    pub camera: Camera,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub projectiles: StructOf<Arena<Projectile>>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let mut model = Self {
            camera: Camera::new(config.camera.fov),
            player: Player::new(config.player),
            actors: StructOf::new(),
            projectiles: StructOf::new(),
            config,
        };
        model.init();
        model
    }

    fn init(&mut self) {
        self.actors.insert(Actor {
            body: Body::new(vec2(5, 0).as_r32(), Shape::Circle { radius: r32(1.0) }),
            health: Health::new(50.0),
            gun: None,
        });
    }
}
