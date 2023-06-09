mod actor;
mod body;
mod collider;
mod projectile;

pub use self::{actor::*, body::*, collider::*, projectile::*};

use super::*;

#[derive(StructOf, Debug)]
pub struct Gasoline {
    pub collider: Collider,
    pub lifetime: Lifetime,
    /// Ignites after being in contact with fire for that time.
    pub ignite_timer: Time,
}

#[derive(StructOf, Debug)]
pub struct Fire {
    pub collider: Collider,
    pub lifetime: Lifetime,
}
