use super::{config::EnemyConfig, *};

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
    pub waves: VecDeque<WaveConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WaveConfig {
    /// The delay between each enemy spawn.
    pub spawn_delay: Time,
    /// Whether to wait for all enemies to be killed before starting the next wave.
    pub wait_for_deaths: bool,
    /// Delay the first enemy spawn.
    pub wave_delay: Time,
    pub enemies: VecDeque<EnemyConfig>,
}

impl WavesConfig {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        file::load_detect(path).await
    }
}
