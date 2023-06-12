mod action;
mod camera;
mod components;
mod effect;
mod health;
mod logic;
mod player;
mod position;
mod shake;
mod waves;
mod weapons;

pub use self::{
    action::*, camera::*, components::*, effect::*, health::*, player::*, position::*, shake::*,
    waves::*, weapons::*,
};

use crate::{
    assets::{config::*, waves::*},
    util::{RealConversions, Vec2RealConversions},
};

use std::collections::VecDeque;

use ecs::{arena::Arena, prelude::*};
use geng::prelude::*;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Id = ecs::arena::Index;
pub type Lifetime = Health;

#[derive(StructOf, Debug, Clone)]
pub struct Explosion {
    pub position: Position,
    pub max_radius: Coord,
    pub lifetime: Lifetime,
}

pub struct Model {
    pub time: Time,
    pub config: Config,
    pub screen_shake: ScreenShake,
    pub camera: Camera,
    pub enemies_list: HashMap<String, EnemyConfig>,
    pub wave_manager: WaveManager,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub gasoline: StructOf<Arena<Gasoline>>,
    pub fire: StructOf<Arena<Fire>>,
    pub explosions: StructOf<Arena<Explosion>>,
    pub queued_effects: VecDeque<QueuedEffect>,
}

impl Model {
    pub fn new(config: Config, enemies: HashMap<String, EnemyConfig>, waves: WavesConfig) -> Self {
        let mut actors = StructOf::new();
        let mut model = Self {
            time: Time::ZERO,
            screen_shake: ScreenShake::new(),
            camera: Camera::new(config.camera.fov),
            player: Player::init(config.player.clone(), &mut actors),
            actors,
            projectiles: StructOf::new(),
            gasoline: StructOf::new(),
            fire: StructOf::new(),
            explosions: StructOf::new(),
            wave_manager: WaveManager::new(waves),
            enemies_list: enemies,
            queued_effects: VecDeque::new(),
            config,
        };
        model.init();
        model
    }

    fn init(&mut self) {}
}
