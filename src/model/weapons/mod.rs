use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ShotPattern {
    Single,
    Multiple { spread_degrees: R32, bullets: usize },
}

#[derive(Debug, Clone)]
pub struct Gun {
    pub config: GunConfig,
    pub shot_delay: Time,
}

impl Gun {
    pub fn new(config: GunConfig) -> Self {
        Self {
            config,
            shot_delay: Time::ZERO,
        }
    }
}

impl Default for ShotPattern {
    fn default() -> Self {
        Self::Single
    }
}
