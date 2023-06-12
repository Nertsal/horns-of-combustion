use super::*;

use crate::model::{Coord, Time};

use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WavesConfig {
    /// The minimal distance to the player, where the enemies' spawn point may be.
    pub min_spawn_distance: Coord,
    /// The maximal distance to the player, where the enemies' spawn point may be.
    pub max_spawn_distance: Coord,
    /// The radius for the spawn circle, in which the all enemies from a wave will spawn.
    pub spawn_circle_radius: Coord,
    pub infinite_wave: InfiniteWaveConfig,
    pub waves: VecDeque<WaveConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfiniteWaveConfig {
    /// How fast the difficulty scales over time.
    pub difficulty_time_scaling: R32,
    /// How much the difficulty scales every wave.
    pub difficulty_wave_scaling: R32,
    /// The delay between each enemy spawn.
    pub spawn_delay: Time,
    /// Delay the first enemy spawn.
    pub wave_delay: Time,
    /// List of enemy names.
    pub enemies: HashMap<String, InfiniteEnemyConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InfiniteEnemyConfig {
    pub cost: R32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveConfig {
    /// The delay between each enemy spawn.
    pub spawn_delay: Time,
    /// Whether to wait for all enemies to be killed before starting the next wave.
    pub wait_for_deaths: bool,
    /// Delay the first enemy spawn.
    pub wave_delay: Time,
    /// List of enemy names.
    pub enemies: VecDeque<String>,
}

impl WavesConfig {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }
}
