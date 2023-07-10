use crate::{model::*, prelude::*};

pub struct UtilRender {
    geng: Geng,
}

impl UtilRender {
    pub fn new(geng: &Geng) -> Self {
        Self { geng: geng.clone() }
    }

    pub fn draw_shape(
        &self,
        shape: Shape,
        transform: mat3<f32>,
        color: Color,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match shape {
            Shape::Circle { radius } => self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::Ellipse::circle(vec2::ZERO, radius.as_f32(), color),
                transform,
            ),
            Shape::Rectangle { width, height } => {
                let size = vec2(width, height).as_f32();
                self.geng.draw2d().draw2d_transformed(
                    framebuffer,
                    camera,
                    &draw2d::Quad::new(Aabb2::ZERO.extend_symmetric(size / 2.0), color),
                    transform,
                )
            }
        }
    }
}
