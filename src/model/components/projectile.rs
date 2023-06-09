use super::*;

#[derive(StructOf, Debug)]
pub struct Projectile {
    pub fraction: Fraction,
    #[structof(nested)]
    pub body: Body,
    pub damage: Hp,
    pub target_pos: Option<vec2<Coord>>,
}

impl Projectile {
    pub fn new(
        pos: vec2<Coord>,
        target: vec2<Coord>,
        fraction: Fraction,
        config: ProjectileConfig,
    ) -> Self {
        Self {
            fraction,
            body: Body::new(pos, config.shape)
                .with_velocity((target - pos).normalize_or_zero() * config.speed),
            damage: config.damage,
            target_pos: None,
        }
    }

    pub fn with_target(self, target_pos: vec2<Coord>) -> Self {
        Self {
            target_pos: Some(target_pos),
            ..self
        }
    }
}
