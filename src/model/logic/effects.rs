use super::*;

impl Model {
    pub(super) fn handle_effects(&mut self, delta_time: Time) {
        while let Some(effect) = self.queued_effects.pop_front() {
            self.handle_effect(effect, delta_time);
        }
    }

    fn handle_effect(&mut self, effect: QueuedEffect, _delta_time: Time) {
        match effect.effect {
            // Effect::Noop => {}
            Effect::ScreenShake(shake) => {
                self.screen_shake.merge(shake);
            }
            Effect::Explosion { position, config } => {
                self.explosions.insert(Explosion {
                    position,
                    max_radius: config.radius,
                    lifetime: Lifetime::new(0.2),
                });

                let check = |body_position: Position| -> bool {
                    let delta = position.direction(body_position, self.config.world_size);
                    let dist = delta.len();
                    dist <= config.radius
                };

                let apply_velocity = |body_position: Position| -> vec2<Coord> {
                    let delta = position.direction(body_position, self.config.world_size);
                    let dist = delta.len();
                    let t = (Coord::ONE
                        - ((dist - Coord::ONE).max(Coord::ZERO) / config.radius).sqrt())
                    .clamp(Coord::ZERO, Coord::ONE);
                    let strength = config.knockback * t;
                    let dir = delta.normalize_or_zero();
                    dir * strength
                };

                let calculate_damage = |body_position: Position, vulnerability: R32| -> Hp {
                    let delta = position.direction(body_position, self.config.world_size);
                    let dist = delta.len();
                    let t = (Coord::ONE
                        - ((dist - Coord::ONE).max(Coord::ZERO) / config.radius).sqrt())
                    .clamp(Coord::ZERO, Coord::ONE);
                    config.damage * t * vulnerability
                };

                {
                    // Update actors
                    #[allow(dead_code)]
                    #[derive(StructQuery)]
                    struct ActorRef<'a> {
                        #[query(storage = ".body.collider")]
                        position: &'a Position,
                        #[query(storage = ".body")]
                        velocity: &'a mut vec2<Coord>,
                        health: &'a mut Health,
                        on_fire: &'a mut Option<OnFire>,
                        stats: &'a Stats,
                    }

                    let mut actor_query = query_actor_ref!(self.actors);
                    let mut actor_iter = actor_query.iter_mut();
                    while let Some((_, actor)) = actor_iter.next() {
                        if !check(*actor.position) {
                            continue;
                        }
                        *actor.velocity += apply_velocity(*actor.position);
                        actor.health.damage(calculate_damage(
                            *actor.position,
                            actor.stats.vulnerability.explosive,
                        ));
                        // Ignite
                        if let Some(mut fire) = config.ignite.clone() {
                            if actor.stats.vulnerability.fire > R32::ZERO {
                                fire.damage_per_second *= actor.stats.vulnerability.fire;
                                *actor.on_fire = Some(update_on_fire(actor.on_fire.clone(), fire));
                            }
                        }
                    }

                    // Screen shake
                    let player_position = actor_query
                        .get(self.player.actor)
                        .map_or(self.camera.center, |player| *player.position);
                    let player_dist = player_position
                        .direction(position, self.config.world_size)
                        .len()
                        .max(r32(0.1));
                    let amplitude = (r32(30.0) / player_dist).clamp_range(r32(0.0)..=r32(30.0));
                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::ScreenShake(ScreenShake {
                            duration: Time::ONE,
                            amplitude,
                        }),
                    });
                }

                {
                    // Update blocks
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

                    let mut block_query = query_block_ref!(self.blocks);
                    let mut block_iter = block_query.iter_mut();
                    while let Some((_, block)) = block_iter.next() {
                        if !check(*block.position) {
                            continue;
                        }
                        block.health.damage(calculate_damage(
                            *block.position,
                            block.vulnerability.explosive,
                        ));
                        // Ignite
                        if let Some(fire) = config.ignite.clone() {
                            *block.on_fire = Some(update_on_fire(block.on_fire.clone(), fire));
                        }
                    }
                }

                if self.config.explosions_affect_projectiles {
                    // Update projectiles
                    #[allow(dead_code)]
                    #[derive(StructQuery)]
                    struct ProjRef<'a> {
                        #[query(storage = ".body.collider")]
                        position: &'a Position,
                        #[query(storage = ".body")]
                        velocity: &'a mut vec2<Coord>,
                    }

                    let mut proj_query = query_proj_ref!(self.projectiles);
                    let mut proj_iter = proj_query.iter_mut();
                    while let Some((_, proj)) = proj_iter.next() {
                        if !check(*proj.position) {
                            continue;
                        }
                        *proj.velocity += apply_velocity(*proj.position);
                    }
                }

                if config.ignite_gasoline {
                    // Ignite gasoline
                    #[allow(dead_code)]
                    #[derive(StructQuery)]
                    struct GasRef<'a> {
                        #[query(optic = ".collider._get", component = "Collider")]
                        position: &'a Position,
                    }

                    let to_ignite: Vec<Id> = query_gas_ref!(self.gasoline)
                        .iter()
                        .filter(|(_, gas)| check(*gas.position))
                        .map(|(id, _)| id)
                        .collect();
                    for id in to_ignite {
                        self.ignite_gasoline(id);
                    }
                }
            }
            Effect::Particles {
                position,
                position_radius,
                velocity,
                size,
                lifetime,
                intensity,
                kind: ai,
            } => {
                let amount = intensity.as_f32().max(0.0).ceil() as usize;
                let mut rng = thread_rng();
                for _ in 0..amount {
                    let pos = rng.gen_circle(vec2::ZERO, position_radius);
                    let pos = position.shifted(pos, self.config.world_size);
                    self.particles.insert(Particle {
                        position: pos,
                        size,
                        velocity,
                        lifetime: Lifetime::new(lifetime),
                        kind: ai,
                    });
                }
            }
        }
    }
}
