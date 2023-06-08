mod camera;
mod components;
mod logic;
mod player;

pub use self::{camera::*, components::*, player::*};

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
    pub bodies: StructOf<Arena<Body>>,
}

impl Model {
    pub fn new(config: Config) -> Self {
        let mut bodies = StructOf::new();
        Self {
            camera: Camera::new(config.camera.fov),
            player: Player::init(&mut bodies),
            bodies,
            config,
        }
    }
}
