use super::util::UtilRender;

use crate::{
    assets::{theme::Theme, Assets},
    model::*,
    util::{Mat3RealConversions, Vec2RealConversions},
};

use ecs::prelude::*;
use geng::prelude::*;

pub struct WorldRender {
    geng: Geng,
    assets: Rc<Assets>,
    theme: Theme,
    util: UtilRender,
}

impl WorldRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, theme: Theme) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            theme,
            util: UtilRender::new(geng),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        // Draw a circle at the center of the world.
        self.util.draw_shape(
            Shape::Circle { radius: r32(10.0) },
            mat3::identity(),
            Color::opaque(0.0, 0.0, 0.3),
            &model.camera,
            framebuffer,
        );

        self.draw_gasoline(model, framebuffer);
        // self.draw_fire(model, framebuffer);
        self.draw_actors(model, framebuffer);
        self.draw_projectiles(model, framebuffer);
        self.draw_health(model, framebuffer);
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

    pub fn draw_fire(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
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
            #[query(storage = ".body")]
            velocity: &'a vec2<Coord>,
            ai: &'a Option<ActorAI>,
        }

        let camera = &model.camera;
        for (id, actor) in &query_actor_ref!(model.actors) {
            if id == model.player.actor {
                // Draw player sprite.
                let sprite = match model.player.state {
                    PlayerState::Human => &self.assets.sprites.player_human,
                    PlayerState::Barrel { .. } => &self.assets.sprites.player_barrel,
                };

                // let position = Aabb2::point(actor.collider.position.as_f32())
                //     .extend_symmetric(sprite_size / 2.0);
                let position = super::pixel_perfect_aabb(
                    actor.collider.position.as_f32(),
                    sprite.size(),
                    camera,
                );

                let circle_radius = r32(1.5) * actor.velocity.len() / r32(30.0);
                let xoff = circle_radius * (model.time * r32(10.0)).cos();
                let yoff = -(circle_radius * (model.time * r32(6.0)).sin()).abs();

                let new_size = position.size() + vec2(xoff.as_f32(), yoff.as_f32());
                let position = Aabb2 {
                    min: vec2(position.center().x - new_size.x / 2.0, position.min.y),
                    max: vec2(
                        position.center().x + new_size.x / 2.0,
                        position.min.y + new_size.y,
                    ),
                };

                self.geng.draw2d().draw2d_transformed(
                    framebuffer,
                    camera,
                    &draw2d::TexturedQuad::new(position, sprite),
                    mat3::rotate_around(
                        position.center(),
                        actor.collider.rotation.as_radians().as_f32(),
                    ),
                );
                continue;
            }

            let color = match actor.ai {
                None => self.theme.player,
                Some(ActorAI::Crawler) => self.theme.enemies.crawler,
                Some(ActorAI::Ranger { .. }) => self.theme.enemies.ranger,
            };
            self.draw_collider(&actor.collider.clone(), color, camera, framebuffer);
        }
    }

    fn draw_health(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            health: &'a Health,
        }

        let camera = &model.camera;
        for (_id, actor) in &query_actor_ref!(model.actors) {
            if actor.health.ratio().as_f32() == 1.0 {
                continue;
            }

            let aabb = actor.collider.clone().compute_aabb();
            let pos = vec2(aabb.center().x, aabb.min.y + aabb.height() * r32(0.9));
            let size = vec2(1.3, 0.4).as_r32();
            let aabb = Aabb2::point(pos).extend_symmetric(size / r32(2.0));
            self.draw_health_bar(aabb, actor.health, camera, framebuffer);
        }
    }

    fn draw_projectiles(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            kind: &'a ProjectileKind,
        }

        let camera = &model.camera;
        for (_id, proj) in &query_proj_ref!(model.projectiles) {
            let sprite = match proj.kind {
                ProjectileKind::Default => &self.assets.sprites.projectile_default,
                ProjectileKind::Orb => &self.assets.sprites.projectile_orb,
                ProjectileKind::SmallOrb => &self.assets.sprites.projectile_small_orb,
                ProjectileKind::SquareSnowflake => &self.assets.sprites.projectile_square_snowflake,
                ProjectileKind::SquidLike => &self.assets.sprites.projectile_squid_like,
                ProjectileKind::WheelPizza => &self.assets.sprites.projectile_wheel_pizza,
            };

            let position =
                super::pixel_perfect_aabb(proj.collider.position.as_f32(), sprite.size(), camera);

            self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::TexturedQuad::new(position, sprite),
                mat3::rotate_around(
                    position.center(),
                    proj.collider.rotation.as_radians().as_f32(),
                ),
            );
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
        self.util.draw_shape(
            collider.shape,
            transform.as_f32(),
            color,
            camera,
            framebuffer,
        )
    }

    fn draw_health_bar(
        &self,
        aabb: Aabb2<Coord>,
        health: &Health,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = mat3::translate(aabb.center()).as_f32();

        self.util.draw_shape(
            Shape::Rectangle {
                width: aabb.width(),
                height: aabb.height(),
            },
            transform,
            self.theme.health_bg,
            camera,
            framebuffer,
        );

        self.util.draw_shape(
            Shape::Rectangle {
                width: aabb.width() * health.ratio() * r32(0.9),
                height: aabb.height() * r32(0.9),
            },
            transform,
            self.theme.health_fg,
            camera,
            framebuffer,
        );
    }
}
