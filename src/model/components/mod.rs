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
    pub fire_radius: Coord,
    pub explosion: ExplosionConfig,
    pub fire: FireConfig,
}

#[derive(StructOf, Debug)]
pub struct Fire {
    pub collider: Collider,
    pub lifetime: Lifetime,
    pub config: FireConfig,
}

#[derive(StructOf, Debug, Clone)]
pub struct Explosion {
    pub position: Position,
    pub max_radius: Coord,
    pub lifetime: Lifetime,
}

#[derive(StructOf, Debug)]
pub struct Particle {
    pub position: Position,
    pub size: Coord,
    pub velocity: vec2<Coord>,
    pub lifetime: Lifetime,
    pub kind: ParticleKind,
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleKind {
    Fire,
    Damage,
    Heal,
    Projectile,
}

#[derive(StructOf, Debug)]
pub struct Block {
    #[structof(nested)]
    pub collider: Collider,
    pub health: Option<Health>,
    pub on_fire: Option<OnFire>,
    pub vulnerability: VulnerabilityStats,
    pub color: Color,
    pub kind: BlockKind,
    pub explosion: Option<ExplosionConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum BlockKind {
    Obstacle,
    Barrel,
}

#[derive(StructOf, Debug)]
pub struct PickUp {
    #[structof(nested)]
    pub body: Body,
    pub kind: PickUpKind,
    pub lifetime: Lifetime,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PickUpKind {
    Heal { hp: Hp },
}
