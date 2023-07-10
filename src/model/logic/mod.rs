mod action;
mod actors;
mod collisions;
mod effects;
mod movement;
mod particles;
mod player;
mod projectiles;
mod waves;
mod weapons;

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) -> Vec<GameEvent> {
        self.time += delta_time;
        if self.actors.health.get(self.player.actor).is_some() {
            self.time_alive = self.time;
        }

        self.update_weapons(delta_time);
        self.update_gas(delta_time);
        self.update_fire(delta_time);
        self.update_explosions(delta_time);
        self.update_on_fire(delta_time);
        self.update_waves(delta_time);

        self.actors_ai(delta_time);
        self.control_player(delta_time);
        self.control_actors(delta_time);
        self.control_projectiles(delta_time);
        self.update_pickups(delta_time);

        self.update_particles(delta_time);
        self.movement(delta_time);
        self.collisions(delta_time);

        self.handle_effects(delta_time);
        self.check_deaths(delta_time);
        self.update_camera(delta_time);

        std::mem::take(&mut self.game_events)
    }

    fn update_explosions(&mut self, delta_time: Time) {
        struct ExplRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut to_remove: Vec<Id> = Vec::new();
        for id in self.explosions.ids() {
            if let Some(expl) = get!(
                self.explosions,
                id,
                ExplRef {
                    lifetime: &mut lifetime
                }
            ) {
                expl.lifetime.change(-delta_time);
                if expl.lifetime.is_min() {
                    to_remove.push(id);
                }
            }
        }

        for id in to_remove {
            self.explosions.remove(id);
        }
    }

    fn check_deaths(&mut self, _delta_time: Time) {
        let mut rng = thread_rng();

        // Actors

        let mut dead_actors: Vec<Id> = query!(self.actors, (&health))
            .filter(|(_, (health,))| health.is_min())
            .map(|(id, _)| id)
            .collect();

        // let mut to_be_spawned: Vec<Projectile> = Vec::new();
        while let Some(id) = dead_actors.pop() {
            let actor = self.actors.remove(id).unwrap();

            // TODO: drop gasoline tank
            self.player.gasoline.change(r32(20.0));

            // Explode
            if let Some(config) = self.config.death_explosion.clone() {
                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::Explosion {
                        position: actor.body.collider.position,
                        config,
                    },
                });

                // // Create a circle of projectiles
                // for i in 0..18 {
                //     to_be_spawned.push(Projectile::new(
                //         actor.body.collider.position,
                //         Angle::from_degrees(r32(i as f32 * 20.0)),
                //         actor.fraction,
                //         ProjectileConfig {
                //             lifetime: r32(10.0),
                //             speed: r32(1.0),
                //             damage: r32(1.0),
                //             shape: Shape::Circle { radius: r32(10.0) },
                //             ai: ProjectileAI::Straight,
                //             kind: ProjectileKind::Orb,
                //             knockback: r32(1.0),
                //         },
                //     ));
                // }
            }

            if let ActorKind::BossBody = actor.kind {
                dead_actors.extend(
                    query!(self.actors, (&kind))
                        .filter(|(_, (kind,))| matches!(kind, ActorKind::BossFoot { .. }))
                        .map(|(id, _)| id),
                );
                let gas_config = &self.config.player.barrel_state.gasoline;
                self.gasoline.insert(Gasoline {
                    collider: Collider::new(
                        Position::zero(self.config.world_size),
                        Shape::Circle { radius: r32(10.0) },
                    ),
                    lifetime: Lifetime::new_max(gas_config.lifetime),
                    ignite_timer: gas_config.ignite_timer,
                    fire_radius: r32(50.0),
                    explosion: gas_config.explosion.clone(),
                    fire: gas_config.fire.clone(),
                });
                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::Explosion {
                        position: Position::zero(self.config.world_size),
                        config: ExplosionConfig {
                            radius: r32(100.0),
                            knockback: r32(200.0),
                            damage: r32(0.0),
                            ignite_gasoline: true,
                            ignite: None,
                        },
                    },
                });
            }

            if rng.gen_bool(self.config.death_drop_heal_chance.as_f32().into()) {
                let config = &self.config.pickups;
                self.pickups.insert(PickUp {
                    body: Body::new(
                        actor.body.collider.position,
                        BodyConfig {
                            shape: Shape::Circle {
                                radius: config.size,
                            },
                            mass: R32::ONE,
                        },
                    ),
                    kind: PickUpKind::Heal {
                        hp: config.heal_amount,
                    },
                    lifetime: Lifetime::new_max(r32(20.0)),
                });
            }
        }

        // // Spawn projectiles
        // for proj in to_be_spawned {
        //     self.projectiles.insert(proj);
        // }

        // Blocks
        let dead_blocks: Vec<Id> = query!(self.blocks, (&health.Get.Some))
            .filter(|(_, (health,))| health.is_min())
            .map(|(id, _)| id)
            .collect();
        for id in dead_blocks {
            let block = self.blocks.remove(id).unwrap();
            if let BlockKind::Barrel = block.kind {
                if let Some(config) = block.explosion {
                    let gas_config = &self.config.player.barrel_state.gasoline;
                    self.gasoline.insert(Gasoline {
                        collider: Collider::new(
                            block.collider.position,
                            Shape::Circle {
                                radius: config.radius / r32(3.0),
                            },
                        ),
                        lifetime: Lifetime::new_max(gas_config.lifetime),
                        ignite_timer: gas_config.ignite_timer,
                        fire_radius: config.radius / r32(3.0),
                        explosion: gas_config.explosion.clone(),
                        fire: gas_config.fire.clone(),
                    });
                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::Explosion {
                            position: block.collider.position,
                            config,
                        },
                    });
                }
                self.add_barrels(1); // Spawn a new barrel
            }
        }
    }

    fn ignite_gasoline(&mut self, gas_id: Id) {
        if let Some(gas) = self.gasoline.remove(gas_id) {
            self.queued_effects.push_back(QueuedEffect {
                effect: Effect::Explosion {
                    position: gas.collider.position,
                    config: gas.explosion,
                },
            });
            self.fire.insert(Fire {
                collider: Collider::new(
                    gas.collider.position,
                    Shape::Circle {
                        radius: gas.fire_radius,
                    },
                ),
                lifetime: Lifetime::new_max(r32(5.0)),
                config: gas.fire,
            });
        }
    }

    fn update_camera(&mut self, delta_time: Time) {
        let scale = 0.15;

        if let Some(player_pos) = self.get_player_pos() {
            // Zoom out if player is moving fast.
            // let player_velocity = self.bodies.get(self.player.body).unwrap().velocity;
            // let player_speed = player_velocity.len();
            // camera.fov = TODO: interpolate fov to player speed.

            // Do not follow player if it is inside the bounds of the camera.
            let direction = self.camera.center.delta_to(player_pos);
            let distance = direction.len();
            if distance > self.camera.fov / r32(3.0) {
                self.player.out_of_view = true;
            }

            if self.player.out_of_view {
                let config = &self.config.camera;
                if distance < config.dead_zone {
                    self.player.out_of_view = false;
                    // camera.target_position = *player_position;
                } else {
                    // Update the target position
                    self.camera.target_position = player_pos;
                    //scale = 0.15;
                }
            }
        }

        // Offset camera towards cursor position
        let cursor_pos_world = self.camera.cursor_pos_world();
        self.camera.offset_center = self.camera.center.delta_to(cursor_pos_world) * r32(scale);

        // Interpolate camera position to the target
        // Take min to not overshoot the target
        self.camera.center.shift(
            (self.camera.center.delta_to(self.camera.target_position))
                * (self.config.camera.speed * delta_time).min(Coord::ONE),
        );

        // Screen shake
        self.screen_shake
            .apply_to_camera(&mut self.camera, delta_time);
        self.screen_shake.update(delta_time);
    }

    fn update_gas(&mut self, delta_time: Time) {
        struct GasRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut expired: Vec<Id> = Vec::new();
        for id in self.gasoline.ids() {
            if let Some(gas) = get!(
                self.gasoline,
                id,
                GasRef {
                    lifetime: &mut lifetime
                }
            ) {
                gas.lifetime.change(-delta_time);
                if gas.lifetime.is_min() {
                    expired.push(id);
                }
            }
        }

        for id in expired {
            self.gasoline.remove(id);
        }
    }

    fn update_fire(&mut self, delta_time: Time) {
        struct FireRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut expired: Vec<Id> = Vec::new();
        for id in self.fire.ids() {
            if let Some(fire) = get!(
                self.fire,
                id,
                FireRef {
                    lifetime: &mut lifetime
                }
            ) {
                fire.lifetime.change(delta_time);
                if fire.lifetime.is_min() {
                    expired.push(id);
                }
            }
        }

        for id in expired {
            self.fire.remove(id);
        }
    }

    fn update_on_fire(&mut self, delta_time: Time) {
        struct ActorRef<'a> {
            position: &'a Position,
            health: &'a mut Health,
            on_fire: &'a mut Option<OnFire>,
            stats: &'a Stats,
        }

        for id in self.actors.ids() {
            let actor = get!(
                self.actors,
                id,
                ActorRef {
                    position: &body.collider.position,
                    health: &mut health,
                    on_fire: &mut on_fire,
                    stats,
                }
            );
            let Some(actor) = actor else { continue; };

            if let Some(on_fire) = actor.on_fire {
                actor.health.change(
                    -on_fire.damage_per_second * actor.stats.vulnerability.fire * delta_time,
                );

                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::Particles {
                        position: *actor.position,
                        position_radius: r32(2.0),
                        velocity: vec2::UNIT_Y,
                        size: r32(0.2),
                        lifetime: r32(1.0),
                        intensity: on_fire.damage_per_second,
                        kind: ParticleKind::Fire,
                    },
                });

                on_fire.duration -= delta_time;
                if on_fire.duration <= Time::ZERO {
                    *actor.on_fire = None;
                }
            }
        }

        struct BlockRef<'a> {
            position: &'a Position,
            health: &'a mut Health,
            on_fire: &'a mut Option<OnFire>,
            vulnerability: &'a VulnerabilityStats,
        }

        for id in self.blocks.ids() {
            let block = get!(
                self.blocks,
                id,
                BlockRef {
                    position: &collider.position,
                    health: &mut health.Get.Some,
                    on_fire: &mut on_fire,
                    vulnerability,
                }
            );
            let Some(block) = block else { continue; };

            if let Some(on_fire) = block.on_fire {
                block
                    .health
                    .change(-on_fire.damage_per_second * block.vulnerability.fire * delta_time);

                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::Particles {
                        position: *block.position,
                        position_radius: r32(1.0),
                        velocity: vec2::UNIT_Y,
                        size: r32(0.1),
                        lifetime: r32(1.0),
                        intensity: on_fire.damage_per_second,
                        kind: ParticleKind::Fire,
                    },
                });

                on_fire.duration -= delta_time;
                if on_fire.duration <= Time::ZERO {
                    *block.on_fire = None;
                }
            }
        }
    }

    fn update_pickups(&mut self, delta_time: Time) {
        struct PickupRef<'a> {
            position: &'a Position,
            velocity: &'a mut vec2<Coord>,
            lifetime: &'a mut Lifetime,
        }

        let player_pos = self.get_player_pos();

        let mut dead_pickups = Vec::new();

        let config = &self.config.pickups;
        for pickup_id in self.pickups.ids() {
            let pickup = get!(
                self.pickups,
                pickup_id,
                PickupRef {
                    position: &body.collider.position,
                    velocity: &mut body.velocity,
                    lifetime: &mut lifetime,
                }
            );
            let Some(pickup) = pickup else { continue; };

            pickup.lifetime.change(-delta_time);

            if pickup.lifetime.is_min() {
                dead_pickups.push(pickup_id);
                continue;
            }

            if let Some(player_pos) = player_pos {
                let delta = pickup.position.delta_to(player_pos);
                let dist = delta.len();
                if dist <= config.attract_radius {
                    let dir = delta.normalize_or_zero();
                    let target_vel = dir * config.max_speed;
                    *pickup.velocity += (target_vel - *pickup.velocity).normalize_or_zero()
                        * config.attract_strength
                        * delta_time;
                }
            }

            // Particles
            self.queued_effects.push_back(QueuedEffect {
                effect: Effect::Particles {
                    position: *pickup.position,
                    position_radius: r32(2.0),
                    velocity: vec2::UNIT_Y,
                    size: r32(0.2),
                    lifetime: r32(1.0),
                    intensity: r32(0.5) * pickup.lifetime.get_ratio().min(r32(0.5)) / r32(0.5),
                    kind: ParticleKind::Heal,
                },
            });
        }

        // Delete dead pickups
        for pickup_id in dead_pickups {
            self.pickups.remove(pickup_id);
        }
    }

    fn get_player_pos(&self) -> Option<Position> {
        self.actors
            .body
            .collider
            .position
            .get(self.player.actor)
            .copied()
    }

    fn get_volume_from(&self, position: Position) -> R32 {
        let player_pos = self.get_player_pos().unwrap_or(self.camera.center);
        let distance = position.distance(player_pos);
        (Coord::ONE / (distance.max(Coord::ONE) / r32(20.0)).sqr()).min(Coord::ONE)
    }
}

fn update_on_fire(status: Option<OnFire>, update: OnFire) -> OnFire {
    let mut on_fire = status.unwrap_or(OnFire {
        duration: Time::ZERO,
        damage_per_second: Hp::ZERO,
    });
    on_fire.duration = on_fire.duration.max(update.duration);
    on_fire.damage_per_second = on_fire.damage_per_second.max(update.damage_per_second);
    on_fire
}
