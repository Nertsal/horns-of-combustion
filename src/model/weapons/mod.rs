use super::*;

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
