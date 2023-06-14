use super::*;

#[derive(Debug, Clone)]
pub struct QueuedEffect {
    pub effect: Effect,
}

#[derive(Debug, Clone)]
pub enum Effect {
    // /// No operation.
    // Noop,
    ScreenShake(ScreenShake),
    Explosion {
        position: Position,
        config: ExplosionConfig,
    },
    Particles {
        position: Position,
        /// Variability in the position.
        position_radius: Coord,
        velocity: vec2<Coord>,
        size: Coord,
        lifetime: Time,
        /// To dynamically control the amount in the settings.
        intensity: R32,
        kind: ParticleKind,
    },
}

impl Effect {
    pub fn particles_damage(position: Position, damage: Hp) -> Self {
        Self::Particles {
            position,
            position_radius: r32(2.0),
            velocity: vec2::UNIT_Y,
            size: r32(0.2),
            lifetime: r32(1.0),
            intensity: damage,
            kind: ParticleKind::Damage,
        }
    }
}
