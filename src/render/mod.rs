use crate::{assets::Assets, model::*, util::Mat3RealConversions};

use ecs::prelude::*;
use geng::prelude::*;

#[allow(dead_code)]
pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        self.draw_bodies(model, framebuffer);
    }

    fn draw_bodies(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            collider: &'a Collider,
        }

        for (_id, body) in &query_body_ref!(model.bodies) {
            let color = Color::BLUE; // TODO
            self.draw_collider(body.collider, color, &model.camera, framebuffer);
        }
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Color,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = collider.transform_mat();
        self.draw_shape(
            collider.shape,
            transform.as_f32(),
            color,
            camera,
            framebuffer,
        )
    }

    fn draw_shape(
        &self,
        shape: Shape,
        transform: mat3<f32>,
        color: Color,
        camera: &Camera2d,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        match shape {
            Shape::Circle { radius } => self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::Ellipse::circle(vec2::ZERO, radius.as_f32(), color),
                transform,
            ),
            Shape::Rectangle { width, height } => todo!(),
        }
    }
}
