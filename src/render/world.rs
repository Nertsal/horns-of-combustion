use super::util::UtilRender;

use crate::{
    assets::{theme::Theme, Assets},
    model::*,
    util::{Mat3RealConversions, Vec2RealConversions},
};

use ecs::{arena::Arena, prelude::*};
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
        self.draw_blocks(model, &model.background_blocks, 1.0, false, framebuffer);

        // Draw a circle at the center of the world.
        let pos = model
            .camera
            .project_f32(Position::ZERO, model.config.world_size);
        self.util.draw_shape(
            Shape::Circle { radius: r32(10.0) },
            mat3::translate(pos),
            self.theme.spawn_circle_color,
            &model.camera,
            framebuffer,
        );

        self.draw_gasoline(model, framebuffer);
        self.draw_blocks(model, &model.blocks, 1.0, true, framebuffer);
        // self.draw_fire(model, framebuffer);
        self.draw_actors(model, framebuffer);
        self.draw_projectiles(model, framebuffer);
        self.draw_pickups(model, framebuffer);
    }

    pub fn draw_ui(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        if model.config.player.barrel_state.gasoline.cost > R32::ZERO {
            self.draw_gasoline_tank(model, framebuffer);
        }
        self.draw_health(model, framebuffer);
        self.draw_enemy_arrows(model, framebuffer);
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
            self.draw_collider(
                &gas.collider.clone(),
                color,
                camera,
                model.config.world_size,
                framebuffer,
            );
        }
    }

    fn draw_blocks(
        &self,
        model: &Model,
        blocks: &StructOf<Arena<Block>>,
        alpha: f32,
        with_outline: bool,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct BlockRef<'a> {
            #[query(nested)]
            collider: &'a Collider,
            color: &'a Color,
            kind: &'a BlockKind,
        }

        let camera = &model.camera;
        for (_, block) in &query_block_ref!(blocks) {
            let collider = block.collider.clone();

            match block.kind {
                BlockKind::Obstacle => {
                    if with_outline {
                        // Outline
                        let outline_color = self.theme.outline_color;
                        let outline_width = r32(0.25);
                        let outline_shape = match collider.shape {
                            Shape::Circle { radius } => Shape::Circle {
                                radius: radius + outline_width,
                            },
                            Shape::Rectangle { width, height } => Shape::Rectangle {
                                width: width + outline_width * r32(2.0),
                                height: height + outline_width * r32(2.0),
                            },
                        };
                        self.draw_collider(
                            &Collider {
                                shape: outline_shape,
                                ..collider
                            },
                            outline_color,
                            camera,
                            model.config.world_size,
                            framebuffer,
                        );
                    }

                    // Fill
                    let mut color = *block.color;
                    color.a *= alpha;
                    self.draw_collider(
                        &collider,
                        color,
                        camera,
                        model.config.world_size,
                        framebuffer,
                    );
                }
                BlockKind::Barrel => {
                    let sprite = &self.assets.sprites.barrel;

                    let pos = camera.project_f32(*block.collider.position, model.config.world_size);
                    let position = super::pixel_perfect_aabb(pos, sprite.size(), camera);

                    self.geng.draw2d().draw2d_transformed(
                        framebuffer,
                        camera,
                        &draw2d::TexturedQuad::new(position, sprite),
                        mat3::rotate_around(
                            position.center(),
                            block.collider.rotation.as_radians().as_f32(),
                        ),
                    );
                }
            }
        }
    }

    pub fn draw_fire(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            collider: &'a Collider,
            lifetime: &'a Lifetime,
        }

        let camera = &model.camera;
        let color = self.theme.fire;
        for (_, fire) in &query_fire_ref!(model.fire) {
            let scale = ((fire.lifetime.max_hp - fire.lifetime.hp).as_f32() / 0.3).clamp(0.0, 1.0);
            self.draw_collider_transformed(
                &fire.collider.clone(),
                color,
                camera,
                model.config.world_size,
                mat3::scale_uniform(scale),
                framebuffer,
            );
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ExplRef<'a> {
            position: &'a Position,
            max_radius: &'a Coord,
            lifetime: &'a Lifetime,
        }

        for (_, expl) in &query_expl_ref!(model.explosions) {
            let radius = (1.0 - expl.lifetime.ratio().as_f32()) * expl.max_radius.as_f32();
            self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::Ellipse::circle_with_cut(vec2::ZERO, radius - 0.5, radius, color),
                mat3::translate(camera.project_f32(*expl.position, model.config.world_size)),
            );
        }

        self.draw_particles(model, framebuffer);

        // Remove fire around the player.
        let player_index = model.player.actor;
        if let Some(player_actor) = model.actors.get(player_index) {
            let player_body = player_actor.body;
            let player_position = player_body.collider.position;

            let aabb = player_body.collider.clone().compute_aabb();
            let radius = aabb.width().max(aabb.height()).as_f32() + 0.2;

            self.geng.draw2d().draw2d(
                framebuffer,
                camera,
                &draw2d::Ellipse::circle(
                    camera.project_f32(*player_position, model.config.world_size),
                    radius,
                    Color::BLACK,
                ),
            );
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
            kind: &'a ActorKind,
        }

        let camera = &model.camera;
        for (_, actor) in &query_actor_ref!(model.actors) {
            // Draw player sprite.
            let sprite = match actor.kind {
                ActorKind::Player => match model.player.state {
                    PlayerState::Human => &self.assets.sprites.player_human,
                    PlayerState::Barrel { .. } => &self.assets.sprites.player_barrel,
                },
                ActorKind::EnemyClown => &self.assets.sprites.enemy_clown,
                ActorKind::EnemyDeathStar => &self.assets.sprites.enemy_death_star,
                ActorKind::EnemyDice => &self.assets.sprites.enemy_dice,
                ActorKind::EnemyHuge => &self.assets.sprites.enemy_huge,
            };

            // let position = Aabb2::point(actor.collider.position.as_f32())
            //     .extend_symmetric(sprite_size / 2.0);
            let position = super::pixel_perfect_aabb(
                camera.project_f32(*actor.collider.position, model.config.world_size),
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

            // let color = match actor.ai {
            //     None => self.theme.player,
            //     Some(ActorAI::Crawler) => self.theme.enemies.crawler,
            //     Some(ActorAI::Ranger { .. }) => self.theme.enemies.ranger,
            // };
            // self.draw_collider(
            //     &actor.collider.clone(),
            //     color,
            //     camera,
            //     model.config.world_size,
            //     framebuffer,
            // );

            let blend_colour = Color::new(1.0, 1.0, 1.0, 1.0);
            self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::TexturedQuad::colored(position, sprite, blend_colour),
                mat3::rotate_around(
                    position.center(),
                    actor.collider.rotation.as_radians().as_f32(),
                ),
            );
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

            let position = super::pixel_perfect_aabb(
                camera.project_f32(*proj.collider.position, model.config.world_size),
                sprite.size(),
                camera,
            );

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

    fn draw_particles(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ParticleRef<'a> {
            position: &'a Position,
            size: &'a Coord,
            lifetime: &'a Lifetime,
            kind: &'a ParticleKind,
        }

        let camera = &model.camera;
        for (_, particle) in &query_particle_ref!(model.particles) {
            let mut color = match particle.kind {
                ParticleKind::Fire => self.theme.fire_particles,
            };
            let alpha = particle.lifetime.ratio().as_f32();
            color.a *= alpha;

            let pos = camera.project_f32(*particle.position, model.config.world_size);
            self.util.draw_shape(
                Shape::Circle {
                    radius: *particle.size,
                },
                mat3::translate(pos),
                color,
                camera,
                framebuffer,
            );
        }
    }

    fn draw_pickups(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PickupRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            kind: &'a PickUpKind,
            lifetime: &'a Lifetime,
        }

        let camera = &model.camera;
        for (_, pickup) in &query_pickup_ref!(model.pickups) {
            let mut color = match pickup.kind {
                PickUpKind::Heal { .. } => self.theme.pickups.heal,
            };
            color.a *= (2.0 * pickup.lifetime.ratio().as_f32()).clamp(0.0, 1.0);
            self.draw_collider(
                &pickup.collider.clone(),
                color,
                camera,
                model.config.world_size,
                framebuffer,
            );
        }
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Color,
        camera: &Camera,
        world_size: vec2<Coord>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        self.draw_collider_transformed(
            collider,
            color,
            camera,
            world_size,
            mat3::identity(),
            framebuffer,
        )
    }

    fn draw_collider_transformed(
        &self,
        collider: &Collider,
        color: Color,
        camera: &Camera,
        world_size: vec2<Coord>,
        transform: mat3<f32>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = collider.transform_mat(camera, world_size).as_f32() * transform;
        self.util.draw_shape(
            collider.shape,
            transform.as_f32(),
            color,
            camera,
            framebuffer,
        )
    }

    fn draw_enemy_arrows(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
        }

        let camera = &model.camera;
        let camera_view = vec2(
            framebuffer.size().as_f32().aspect() * camera.fov.as_f32(),
            camera.fov.as_f32(),
        ) / 2.0;

        for (_id, actor) in &query_actor_ref!(model.actors) {
            let delta = camera
                .center
                .direction(*actor.position, model.config.world_size)
                .as_f32();
            if delta.x.abs() < camera_view.x && delta.y.abs() < camera_view.y {
                // In view
                continue;
            }

            let camera_view = camera_view - vec2::splat(2.0); // Margin
            let x = delta.x.clamp_abs(camera_view.x);
            let y = delta.y.clamp_abs(camera_view.y);

            let texture = &self.assets.sprites.arrow;

            let angle = delta.arg();
            let pos = camera.center.to_world_f32() + vec2(x, y);
            let pos = super::pixel_perfect_aabb(pos, texture.size(), camera);
            let color = Color::WHITE;
            self.geng.draw2d().draw2d_transformed(
                framebuffer,
                camera,
                &draw2d::TexturedQuad::colored(pos, texture, color),
                mat3::rotate_around(pos.center(), angle),
            );
        }
    }

    fn draw_gasoline_tank(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        // let screen = framebuffer.size().as_f32();
        let camera = &geng::PixelPerfectCamera;
        let size = vec2(20.0, 30.0);
        let aabb = Aabb2::point(vec2::splat(20.0)).extend_positive(size);
        self.geng
            .draw2d()
            .draw2d(framebuffer, camera, &draw2d::Quad::new(aabb, Rgba::BLACK));

        let t = model.player.gasoline.ratio().as_f32();
        let aabb = Aabb2::point(aabb.bottom_left())
            .extend_positive(vec2(aabb.width(), aabb.height() * t))
            .extend_uniform(-1.0);
        self.geng.draw2d().draw2d(
            framebuffer,
            camera,
            &draw2d::Quad::new(aabb, self.theme.gasoline),
        );
    }

    fn draw_health(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            health: &'a Health,
            fraction: &'a Fraction,
        }

        let camera = &model.camera;
        for (_id, actor) in &query_actor_ref!(model.actors) {
            if actor.health.ratio().as_f32() == 1.0 {
                continue;
            }

            let aabb = actor.collider.clone().compute_aabb();
            let radius = aabb.width().max(aabb.height()).as_f32() + 0.2;
            let pos = camera.project_f32(*actor.collider.position, model.config.world_size);
            let color = match actor.fraction {
                Fraction::Player => self.theme.health_fg_player,
                Fraction::Enemy => self.theme.health_fg_enemy,
            };
            self.draw_health_arc(pos, radius, actor.health, color, camera, framebuffer);
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct BlockRef<'a> {
            #[query(nested)]
            collider: &'a Collider,
            #[query(optic = "._Some")]
            health: &'a Health,
        }

        for (_id, actor) in &query_block_ref!(model.blocks) {
            if actor.health.ratio().as_f32() == 1.0 {
                continue;
            }

            let aabb = actor.collider.clone().compute_aabb();
            let radius = aabb.width().max(aabb.height()).as_f32() + 0.2;
            let pos = camera.project_f32(*actor.collider.position, model.config.world_size);
            self.draw_health_arc(
                pos,
                radius,
                actor.health,
                self.theme.health_fg_enemy,
                camera,
                framebuffer,
            );
        }
    }

    fn draw_health_arc(
        &self,
        center: vec2<f32>,
        radius: f32,
        health: &Health,
        color: Color,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let transform = mat3::translate(center) * mat3::scale_uniform(radius);

        ugli::draw(
            framebuffer,
            &self.assets.shaders.health_arc,
            ugli::DrawMode::TriangleFan,
            &super::unit_geometry(self.geng.ugli()),
            (
                ugli::uniforms! {
                    u_health: health.ratio().as_f32(),
                    u_color: color,
                    u_color_bg: self.theme.health_bg,
                    u_model_matrix: transform,
                    u_width: 0.1,
                },
                camera.uniforms(framebuffer.size().as_f32()),
            ),
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::straight_alpha()),
                ..default()
            },
        );
    }
}
