mod action;
mod camera;
mod components;
mod health;
mod logic;
mod player;
mod waves;
mod weapons;

pub use self::{action::*, camera::*, components::*, health::*, player::*, waves::*, weapons::*};

use crate::{
    assets::{config::*, waves::*},
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
    pub enemies_list: HashMap<String, EnemyConfig>,
    pub wave_manager: WaveManager,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub gasoline: StructOf<Arena<Gasoline>>,
    pub fire: StructOf<Arena<Fire>>,
}

impl Model {
    pub fn new(config: Config, enemies: HashMap<String, EnemyConfig>, waves: WavesConfig) -> Self {
        let mut actors = StructOf::new();
        let mut model = Self {
            camera: Camera::new(config.camera.fov),
            player: Player::init(config.player.clone(), &mut actors),
            actors,
            projectiles: StructOf::new(),
            gasoline: StructOf::new(),
            fire: StructOf::new(),
            wave_manager: WaveManager::new(waves),
            enemies_list: enemies,
            config,
        };
        model.init();
        model
    }

    fn init(&mut self) {}
}
