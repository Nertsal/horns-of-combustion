use super::*;

impl Model {
    pub(super) fn generate_level(&mut self) {
        let config = &self.config.level;

        let mut rng = thread_rng();
        let mut spawns: Vec<Position> = Vec::new();

        const MAX_ITER: usize = 100;
        for _ in 0..MAX_ITER {
            if spawns.len() >= config.blocks_number {
                break;
            }

            let position = Position::random(&mut rng, self.config.world_size);
        }
    }
}
