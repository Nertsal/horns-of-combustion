use super::*;

#[derive(Debug)]
pub struct Camera {
    pub center: vec2<Coord>,
    pub fov: Coord,
    pub target_position: vec2<Coord>,
}

impl Camera {
    pub fn new(fov: impl Float) -> Self {
        Self {
            center: vec2::ZERO,
            fov: fov.as_r32(),
            target_position: vec2::ZERO,
        }
    }

    pub fn to_camera2d(&self) -> geng::Camera2d {
        geng::Camera2d {
            center: self.center.as_f32(),
            rotation: 0.0,
            fov: self.fov.as_f32(),
        }
    }
}
