use super::*;

impl Model {
    pub(super) fn generate_level(&mut self) {
        let config = &self.level;
        let palette = self.theme.get_palette(&self.theme.level.foreground);
        generate_blocks(
            &config.foreground,
            &palette,
            self.config.world_size,
            &mut self.blocks,
        );

        let palette = self.theme.get_palette(&self.theme.level.background);
        generate_blocks(
            &config.background,
            &palette,
            self.config.world_size,
            &mut self.background_blocks,
        );
    }

    pub(super) fn add_barrels(&mut self, amount: usize) {
        let barrel = self
            .level
            .foreground
            .blocks
            .first()
            .expect("No foreground objects found") // TODO: better
            .clone();
        assert!(
            matches!(barrel.kind, BlockKind::Barrel),
            "First block in level foreground config expected to be a barrel"
        );
        let config = ProcGenConfig {
            spacing: self.level.foreground.spacing,
            blocks_number: amount,
            blocks: vec![barrel],
        };
        let palette = self.theme.get_palette(&self.theme.level.foreground);
        generate_blocks(&config, &palette, self.config.world_size, &mut self.blocks);
    }
}

fn generate_blocks(
    config: &ProcGenConfig,
    palette: &[Color],
    world_size: vec2<Coord>,
    result: &mut StructOf<Arena<Block>>,
) {
    let mut rng = thread_rng();

    let max_iter = config.blocks_number * 5; // ~5 attempts per block
    let mut added = 0;
    for _ in 0..max_iter {
        if added >= config.blocks_number {
            break;
        }

        let position = Position::random(&mut rng, world_size);
        if result
            .collider
            .position
            .iter()
            .any(|(_, pos)| pos.distance(position, world_size) < config.spacing)
        {
            // Too close to another block
            continue;
        }

        let block = config
            .blocks
            .choose_weighted(&mut rng, |config| config.weight.as_f32())
            .expect("no block variants found to generate")
            .clone();

        let (color, rotation) = match block.kind {
            BlockKind::Obstacle => (
                *palette.choose(&mut rng).expect("no colors in the pallete"),
                Angle::from_degrees(rng.gen_range(0.0..360.0).as_r32()),
            ),
            BlockKind::Barrel => (Color::WHITE, Angle::ZERO),
        };

        result.insert(Block {
            color,
            health: block.health.map(Health::new),
            on_fire: None,
            vulnerability: block.vulnerability,
            kind: block.kind,
            collider: {
                Collider {
                    position,
                    rotation,
                    shape: block.shape,
                }
            },
            explosion: block.explosion,
        });
        added += 1;
    }
}
