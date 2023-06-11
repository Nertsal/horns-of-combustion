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
        position: vec2<Coord>,
        radius: Coord,
        strength: Coord,
    },
}
