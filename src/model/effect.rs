use super::*;

#[derive(Debug, Clone)]
pub struct QueuedEffect {
    pub effect: Effect,
}

#[derive(Debug, Clone)]
pub enum Effect {
    /// No operation.
    Noop,
    ScreenShake(ScreenShake),
    Explosion {
        position: Position,
        radius: Coord,
        strength: Coord,
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
