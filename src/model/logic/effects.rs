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

                    // Damage
                    let delta = position.direction(*actor.position, self.config.world_size);
                    let dist = delta.len();
                    let t = (Coord::ONE
                        - ((dist - Coord::ONE).max(Coord::ZERO) / config.radius).sqrt())
                    .clamp(Coord::ZERO, Coord::ONE);
                    let damage = config.damage * t;
                    actor.health.damage(damage);

                    // Ignite
                    if let Some(fire) = config.ignite.clone() {
                        if !actor.stats.fire_immune {
                            *actor.on_fire = Some(update_on_fire(actor.on_fire.clone(), fire));
                        }
                    }
                }

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

                // TODO: account for distance
                let player = actor_query
                    .get(self.player.actor)
                    .expect("Player actor not found");
                let player_dist = player
                    .position
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
