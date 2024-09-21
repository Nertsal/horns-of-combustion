mod action;
mod camera;
mod components;
mod effect;
mod gen;
mod logic;
mod player;
mod shake;
mod waves;
mod weapons;

pub use self::{
    action::*, camera::*, components::*, effect::*, player::*, shake::*, waves::*, weapons::*,
};

use crate::{
    assets::{config::*, theme::Theme, waves::*},
    game::{GameEvent, Sound},
    prelude::*,
};

use std::collections::VecDeque;

use geng_utils::bounded::Bounded;

pub type Color = Rgba<f32>;
pub type Time = R32;
pub type Coord = R32;
pub type Lifetime = Bounded<Time>;
pub type Hp = R32;
pub type Health = Bounded<Hp>;

pub struct Model {
    pub theme: Theme,
    pub time: Time,
    pub time_alive: Time,
    pub config: Config,
    pub level: LevelConfig,
    pub waves: WavesConfig,
    pub screen_shake: ScreenShake,
    pub camera: Camera,
    pub enemies_list: HashMap<String, EnemyConfig>,
    pub wave_manager: WaveManager,
    pub player: Player,
    pub actors: StructOf<Arena<Actor>>,
    pub blocks: StructOf<Arena<Block>>,
    pub background_blocks: StructOf<Arena<Block>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub gasoline: StructOf<Arena<Gasoline>>,
    pub fire: StructOf<Arena<Fire>>,
    pub explosions: StructOf<Arena<Explosion>>,
    pub particles: StructOf<Arena<Particle>>,
    pub pickups: StructOf<Arena<PickUp>>,
    pub queued_effects: VecDeque<QueuedEffect>,
    pub game_events: Vec<GameEvent>,
}

impl Model {
    pub fn new(
        theme: Theme,
        config: Config,
        level: LevelConfig,
        enemies: HashMap<String, EnemyConfig>,
        waves: WavesConfig,
    ) -> Self {
        let mut actors = StructOf::<Arena<Actor>>::default();
        let mut model = Self {
            theme,
            time: Time::ZERO,
            time_alive: Time::ZERO,
            screen_shake: ScreenShake::new(),
            camera: Camera::new(config.camera.fov, config.world_size),
            player: Player::init(config.player.clone(), config.world_size, &mut actors),
            actors,
            blocks: default(),
            background_blocks: default(),
            projectiles: default(),
            gasoline: default(),
            fire: default(),
            explosions: default(),
            particles: default(),
            pickups: default(),
            wave_manager: WaveManager::new(waves.clone(), config.world_size),
            enemies_list: enemies,
            queued_effects: VecDeque::new(),
            game_events: Vec::new(),
            config,
            level,
            waves,
        };
        model.init();
        model
    }

    fn init(&mut self) {
        // TODO: navmesh
        self.generate_level();
    }

    /// Revive the player.
    pub fn revive(&mut self) {
        self.player = Player::init(
            self.config.player.clone(),
            self.config.world_size,
            &mut self.actors,
        );
    }

    /// Restart the whole game.
    pub fn reset(&mut self) {
        *self = Self::new(
            self.theme.clone(),
            self.config.clone(),
            self.level.clone(),
            self.enemies_list.clone(),
            self.waves.clone(),
        );
    }
}
