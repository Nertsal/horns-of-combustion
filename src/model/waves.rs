use std::collections::VecDeque;

use super::*;

#[derive(Debug)]
pub struct WaveManager {
    pub config: WavesConfig,
    pub current_wave: WaveConfig,
    pub wave_delay: Time,
    pub spawn_delay: Time,
    // pub current_enemies: Vec<Id>,
    /// The point for spawning the wave's enemies around.
    pub spawn_point: Position,
}

impl WaveManager {
    pub fn new(config: WavesConfig) -> Self {
        Self {
            wave_delay: Time::ZERO,
            spawn_delay: Time::ZERO,
            // current_enemies: Vec::new(),
            current_wave: WaveConfig {
                spawn_delay: Time::ZERO,
                wait_for_deaths: false,
                wave_delay: Time::ZERO,
                enemies: VecDeque::new(),
            },
            config,
            spawn_point: Position::ZERO,
        }
    }
}
