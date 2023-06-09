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
}

#[derive(StructOf, Debug)]
pub struct Fire {
    pub collider: Collider,
    pub lifetime: Lifetime,
}
