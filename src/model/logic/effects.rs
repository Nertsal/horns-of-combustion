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
                    lifetime: Lifetime::new_max(r32(0.2)),
                });

                let check = |body_position: Position| -> bool {
                    let delta = position.delta_to(body_position);
                    let dist = delta.len();
                    dist <= config.radius
                };

                let apply_velocity = |body_position: Position| -> vec2<Coord> {
                    let delta = position.delta_to(body_position);
                    let dist = delta.len();
                    let t = (Coord::ONE
                        - ((dist - Coord::ONE).max(Coord::ZERO) / config.radius).sqrt())
                    .clamp(Coord::ZERO, Coord::ONE);
                    let strength = config.knockback * t;
                    let dir = delta.normalize_or_zero();
                    dir * strength
                };

                let calculate_damage = |body_position: Position, vulnerability: R32| -> Hp {
                    let delta = position.delta_to(body_position);
                    let dist = delta.len();
                    let t = (Coord::ONE
                        - ((dist - Coord::ONE).max(Coord::ZERO) / config.radius).sqrt())
                    .clamp(Coord::ZERO, Coord::ONE);
                    config.damage * t * vulnerability
                };

                {
                    // Update actors
                    struct ActorRef<'a> {
                        position: &'a Position,
                        velocity: &'a mut vec2<Coord>,
                        health: &'a mut Health,
                        on_fire: &'a mut Option<OnFire>,
                        stats: &'a Stats,
                    }

                    for actor_id in self.actors.ids() {
                        let actor = get!(
                            self.actors,
                            actor_id,
                            ActorRef {
                                position: &body.collider.position,
                                velocity: &mut body.velocity,
                                health: &mut health,
                                on_fire: &mut on_fire,
                                stats,
                            }
                        );
                        let Some(actor) = actor else { continue };

                        if !check(*actor.position) {
                            continue;
                        }
                        *actor.velocity += apply_velocity(*actor.position);
                        actor.health.change(-calculate_damage(
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
                    let player_position = self.get_player_pos().unwrap_or(self.camera.center);
                    let player_dist = player_position.distance(position).max(r32(0.1));
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
                    struct BlockRef<'a> {
                        position: &'a Position,
                        health: &'a mut Health,
                        on_fire: &'a mut Option<OnFire>,
                        vulnerability: &'a VulnerabilityStats,
                    }

                    for block_id in self.blocks.ids() {
                        let block = get!(
                            self.blocks,
                            block_id,
                            BlockRef {
                                position: &collider.position,
                                health: &mut health.Get.Some,
                                on_fire: &mut on_fire,
                                vulnerability,
                            }
                        );
                        let Some(block) = block else { continue };

                        if !check(*block.position) {
                            continue;
                        }
                        block.health.change(-calculate_damage(
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
                    struct ProjRef<'a> {
                        position: &'a Position,
                        velocity: &'a mut vec2<Coord>,
                    }

                    for proj_id in self.projectiles.ids() {
                        if let Some(proj) = get!(
                            self.projectiles,
                            proj_id,
                            ProjRef {
                                position: &body.collider.position,
                                velocity: &mut body.velocity,
                            }
                        ) {
                            if !check(*proj.position) {
                                continue;
                            }
                            *proj.velocity += apply_velocity(*proj.position);
                        }
                    }
                }

                if config.ignite_gasoline {
                    // Ignite gasoline
                    let to_ignite: Vec<Id> = query!(self.gasoline, (&collider.position))
                        .filter(|(_, (&pos,))| check(pos))
                        .map(|(id, _)| id)
                        .collect();
                    for id in to_ignite {
                        self.ignite_gasoline(id);
                    }
                }

                // Sound
                self.game_events.push(GameEvent::PlaySound {
                    sound: Sound::Explosion,
                    volume: self.get_volume_from(position),
                });
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
                let mut rng = thread_rng();
                let amount = if intensity.as_f32() < 1.0 {
                    usize::from(rng.gen_bool(intensity.as_f32().into()))
                } else {
                    #[allow(clippy::cast_sign_loss)]
                    // `.max(0.0)` makes sure the value is not negative
                    {
                        intensity.as_f32().ceil().max(0.0) as usize
                    }
                };
                for _ in 0..amount {
                    let pos = rng.gen_circle(vec2::ZERO, position_radius);
                    let pos = position.shifted(pos);
                    self.particles.insert(Particle {
                        position: pos,
                        size,
                        velocity,
                        lifetime: Lifetime::new_max(lifetime),
                        kind: ai,
                    });
                }
            }
        }
    }
}
