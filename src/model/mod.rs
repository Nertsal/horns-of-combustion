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
        for (i, config) in self.config.enemies.iter() {
            self.actors
                .insert(Actor::new_enemy(vec2(15, i * 3).as_r32(), config.clone()));
        }
    }
}
