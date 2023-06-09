use crate::{
    assets::{theme::Theme, Assets},
    model::*,
    util::{Mat3RealConversions, Vec2RealConversions},
};

use ecs::prelude::*;
use geng::prelude::*;

#[allow(dead_code)]
pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
    theme: Theme,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, theme: Theme) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            theme,
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(self.theme.background), None, None);

        // Draw a circle at the center of the world.
        self.draw_shape(
            Shape::Circle { radius: r32(10.0) },
            mat3::identity(),
            Color::opaque(0.0, 0.0, 0.3),
            &model.camera,
            framebuffer,
        );

        self.draw_gasoline(model, framebuffer);
        self.draw_fire(model, framebuffer);
        self.draw_actors(model, framebuffer);
        self.draw_projectiles(model, framebuffer);
    }

    fn draw_gasoline(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
        }

        let camera = &model.camera;
        let color = self.theme.gasoline;
        for (_, gas) in &query_gas_ref!(model.gasoline) {
            self.draw_collider(&gas.collider.clone(), color, camera, framebuffer);
        }
    }

    fn draw_fire(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            collider: &'a Collider,
        }

        let camera = &model.camera;
        let color = self.theme.fire;
        for (_, fire) in &query_fire_ref!(model.fire) {
            self.draw_collider(&fire.collider.clone(), color, camera, framebuffer);
        }
    }

    fn draw_actors(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            ai: &'a Option<ActorAI>,
        }

        let camera = &model.camera;
        for (_id, actor) in &query_actor_ref!(model.actors) {
            let color = match actor.ai {
                None => self.theme.player,
                Some(ActorAI::Crawler) => self.theme.enemies.crawler,
            };
            self.draw_collider(&actor.collider.clone(), color, camera, framebuffer);
        }
    }

    fn draw_projectiles(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
        }

        let camera = &model.camera;
        let color = self.theme.projectile;
        for (_id, proj) in &query_proj_ref!(model.projectiles) {
            self.draw_collider(&proj.collider.clone(), color, camera, framebuffer);
        }
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Color,
        camera: &Camera,
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
