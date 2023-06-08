mod action;
mod camera;
mod components;
mod logic;
mod player;
mod weapons;

pub use self::{action::*, camera::*, components::*, player::*, weapons::*};

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
        Self {
            camera: Camera::new(config.camera.fov),
            player: Player::new(config.player),
            actors: StructOf::new(),
            projectiles: StructOf::new(),
            config,
        }
    }
}
