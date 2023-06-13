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
    pub fn update(&mut self, delta_time: Time) {
        self.time += delta_time;

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
        self.attract_pickups(delta_time);

        self.update_particles(delta_time);
        self.movement(delta_time);
        self.collisions(delta_time);

        self.handle_effects(delta_time);
        self.check_deaths(delta_time);
        self.update_camera(delta_time);
    }

    fn update_explosions(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ExplRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut query = query_expl_ref!(self.explosions);

        let mut to_remove: Vec<Id> = Vec::new();
        let mut iter = query.iter_mut();
        while let Some((id, expl)) = iter.next() {
            expl.lifetime.damage(delta_time);
            if expl.lifetime.is_dead() {
                to_remove.push(id);
            }
        }

        for id in to_remove {
            self.explosions.remove(id);
        }
    }

    fn check_deaths(&mut self, _delta_time: Time) {
        // Actors
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            health: &'a Health,
        }

        let mut rng = thread_rng();

        let dead_actors: Vec<Id> = query_actor_ref!(self.actors)
            .iter()
            .filter(|(_, actor)| actor.health.is_dead())
            .map(|(id, _)| id)
            .collect();
        for id in dead_actors {
            let actor = self.actors.remove(id).unwrap();

            // TODO: drop gasoline tank
            self.player.gasoline.heal(r32(20.0));

            // Explode
            if let Some(config) = self.config.death_explosion.clone() {
                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::Explosion {
                        position: actor.body.collider.position,
                        config,
                    },
                });
            }

            if rng.gen_bool(self.config.death_drop_heal_chance.as_f32().into()) {
                let config = &self.config.pickups;
                self.pickups.insert(PickUp {
                    body: Body::new(
                        actor.body.collider.position,
                        Shape::Circle {
                            radius: config.size,
                        },
                    ),
                    kind: PickUpKind::Heal {
                        hp: config.heal_amount,
                    },
                });
            }
        }

        // Blocks
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct BlockRef<'a> {
            #[query(optic = "._Some")]
            health: &'a mut Health,
        }

        let dead_blocks: Vec<Id> = query_block_ref!(self.blocks)
            .iter()
            .filter(|(_, block)| block.health.is_dead())
            .map(|(id, _)| id)
            .collect();
        for id in dead_blocks {
            let block = self.blocks.remove(id).unwrap();
            if let BlockKind::Barrel = block.kind {
                if let Some(config) = block.explosion {
                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::Explosion {
                            position: block.collider.position,
                            config,
                        },
                    });
                }
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
                lifetime: Lifetime::new(5.0),
                config: gas.fire,
            });
        }
    }

    fn update_camera(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
        }

        let query = query_player_ref!(self.actors);
        let player = query
            .get(self.player.actor)
            .expect("Player actor not found");

        let camera = &mut self.camera;

        // Zoom out if player is moving fast.
        // let player_velocity = self.bodies.get(self.player.body).unwrap().velocity;
        // let player_speed = player_velocity.len();
        // camera.fov = TODO: interpolate fov to player speed.

        // Do not follow player if it is inside the bounds of the camera.
        let direction = camera
            .center
            .direction(*player.position, self.config.world_size);
        let distance = direction.len();
        if distance > camera.fov / r32(3.0) {
            self.player.out_of_view = true;
        }

        if self.player.out_of_view {
            let config = &self.config.camera;
            if distance < config.dead_zone {
                self.player.out_of_view = false;
                // camera.target_position = *player_position;
            } else {
                // Update the target position
                camera.target_position = *player.position;
            }

            // Interpolate camera position to the target
            // Take min to not overshoot the target
            camera.center.shift(
                direction * (config.speed * delta_time).min(Coord::ONE),
                self.config.world_size,
            );
        }

        // Screen shake
        self.screen_shake
            .apply_to_camera(camera, self.config.world_size, delta_time);
        self.screen_shake.update(delta_time);
    }

    fn update_gas(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut query = query_gas_ref!(self.gasoline);
        let mut iter = query.iter_mut();
        let mut expired: Vec<Id> = Vec::new();
        while let Some((id, gas)) = iter.next() {
            gas.lifetime.damage(delta_time);
            if gas.lifetime.is_dead() {
                expired.push(id);
            }
        }

        for id in expired {
            self.gasoline.remove(id);
        }
    }

    fn update_fire(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut query = query_fire_ref!(self.fire);
        let mut iter = query.iter_mut();
        let mut expired: Vec<Id> = Vec::new();
        while let Some((id, fire)) = iter.next() {
            fire.lifetime.damage(delta_time);
            if fire.lifetime.is_dead() {
                expired.push(id);
            }
        }

        for id in expired {
            self.fire.remove(id);
        }
    }

    fn update_on_fire(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
            health: &'a mut Health,
            on_fire: &'a mut Option<OnFire>,
            stats: &'a Stats,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            if let Some(on_fire) = actor.on_fire {
                actor.health.damage(
                    on_fire.damage_per_second * actor.stats.vulnerability.fire * delta_time,
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

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct BlockRef<'a> {
            #[query(storage = ".collider")]
            position: &'a Position,
            #[query(optic = "._Some")]
            health: &'a mut Health,
            on_fire: &'a mut Option<OnFire>,
            vulnerability: &'a VulnerabilityStats,
        }

        let mut query = query_block_ref!(self.blocks);
        let mut iter = query.iter_mut();
        while let Some((_, block)) = iter.next() {
            if let Some(on_fire) = block.on_fire {
                block
                    .health
                    .damage(on_fire.damage_per_second * block.vulnerability.fire * delta_time);

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

    fn attract_pickups(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PickupRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a Position,
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
        }

        let player_query = query_player_ref!(self.actors);
        let Some(player) = player_query.get(self.player.actor) else {
            return;
        };

        let mut pickup_query = query_pickup_ref!(self.pickups);
        let mut pickup_iter = pickup_query.iter_mut();

        let config = &self.config.pickups;
        while let Some((_, pickup)) = pickup_iter.next() {
            let delta = pickup
                .position
                .direction(*player.position, self.config.world_size);
            let dist = delta.len();
            if dist <= config.attract_radius {
                let dir = delta.normalize_or_zero();
                let target_vel = dir * config.max_speed;
                *pickup.velocity += (target_vel - *pickup.velocity).normalize_or_zero()
                    * config.attract_strength
                    * delta_time;
            }
        }
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
