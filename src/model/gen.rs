use super::*;

impl Model {
    pub(super) fn generate_level(&mut self) {
        let config = &self.config.level;

        let mut rng = thread_rng();
        let mut spawns: Vec<Position> = Vec::new();

        let max_iter = config.blocks_number * 3; // ~3 attempts per block
        for _ in 0..max_iter {
            if spawns.len() >= config.blocks_number {
                break;
            }

            let position = Position::random(&mut rng, self.config.world_size);
            if spawns
                .as_slice()
                .iter()
                .any(|pos| pos.distance(position, self.config.world_size) < config.spacing)
            {
                // Too close to another block
                continue;
            }

            let block = config
                .blocks
                .choose_weighted(&mut rng, |config| config.weight.as_f32())
                .expect("no block variants found to generate");

            let (color, rotation) = match block.kind {
                BlockKind::Obstacle => (
                    *self
                        .pallete
                        .values()
                        .choose(&mut rng)
                        .expect("no colors in the pallete"),
                    Angle::from_degrees(rng.gen_range(0.0..360.0).as_r32()),
                ),
                BlockKind::Barrel => (Color::WHITE, Angle::ZERO),
            };

            spawns.push(position);
            self.blocks.insert(Block {
                color,
                health: block.health.map(Health::new),
                kind: block.kind,
                collider: {
                    Collider {
                        position,
                        rotation,
                        shape: block.shape,
                    }
                },
                explosion: block.explosion.clone(),
            });
        }
    }
}
