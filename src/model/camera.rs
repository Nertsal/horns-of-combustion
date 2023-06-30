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
    pub fn new(fov: impl Float, world_size: vec2<Coord>) -> Self {
        Self {
            center: Position::zero(world_size),
            offset_center: vec2::ZERO,
            fov: fov.as_r32(),
            target_position: Position::zero(world_size),
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
    pub fn project(&self, position: Position) -> vec2<Coord> {
        let center = self.center.to_world();
        center + self.center.delta_to(position)
    }

    /// Project a world position to a position relative to the camera.
    pub fn project_f32(&self, position: Position) -> vec2<f32> {
        self.project(position).as_f32()
    }

    /// Returns the positions of the cursor in the world space.
    pub fn cursor_pos_world(&self) -> Position {
        let pos = self
            .screen_to_world(self.framebuffer_size.as_f32(), self.cursor_pos.as_f32())
            .as_r32();
        Position::from_world(pos, self.center.world_size())
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
