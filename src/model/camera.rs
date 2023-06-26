use super::*;

#[derive(Debug)]
pub struct Camera {
    pub center: Position,
    pub offset_center: vec2<Coord>,
    pub fov: Coord,
    pub target_position: Position,
    pub cursor_pos: vec2<f64>,
    pub framebuffer_size: vec2<usize>,
}

impl Camera {
    pub fn new(fov: impl Float) -> Self {
        Self {
            center: Position::ZERO,
            offset_center: vec2::ZERO,
            fov: fov.as_r32(),
            target_position: Position::ZERO,
            cursor_pos: vec2::ZERO,
            framebuffer_size: vec2(1, 1),
        }
    }

    fn to_camera2d(&self) -> geng::Camera2d {
        geng::Camera2d {
            center: self.center.to_world().as_f32() + self.offset_center.as_f32(),
            rotation: 0.0,
            fov: self.fov.as_f32(),
        }
    }

    /// Project a world position to a position relative to the camera.
    pub fn project(&self, position: Position, world_size: vec2<Coord>) -> vec2<Coord> {
        let center = self.center.to_world();
        center + self.center.direction(position, world_size)
    }

    /// Project a world position to a position relative to the camera.
    pub fn project_f32(&self, position: Position, world_size: vec2<Coord>) -> vec2<f32> {
        self.project(position, world_size).as_f32()
    }

    ///
    pub fn cursor_pos_world(&self) -> vec2<R32> {
        self.screen_to_world(self.framebuffer_size.as_f32(), self.cursor_pos.as_f32())
            .as_r32()
    }
}

impl geng::AbstractCamera2d for Camera {
    fn view_matrix(&self) -> mat3<f32> {
        self.to_camera2d().view_matrix()
    }

    fn projection_matrix(&self, framebuffer_size: vec2<f32>) -> mat3<f32> {
        self.to_camera2d().projection_matrix(framebuffer_size)
    }
}
