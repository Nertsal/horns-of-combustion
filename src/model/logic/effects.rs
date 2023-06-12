use super::*;

impl Model {
    pub(super) fn handle_effects(&mut self, delta_time: Time) {
        while let Some(effect) = self.queued_effects.pop_front() {
            self.handle_effect(effect, delta_time);
        }
    }

    fn handle_effect(&mut self, effect: QueuedEffect, _delta_time: Time) {
        match effect.effect {
            Effect::Noop => {}
            Effect::ScreenShake(shake) => {
                self.screen_shake.merge(shake);
            }
            Effect::Explosion {
                position,
                radius,
                strength,
            } => {
                self.explosions.insert(Explosion {
                    position,
                    max_radius: radius,
                    lifetime: Lifetime::new(0.2),
                });

                #[allow(dead_code)]
                #[derive(StructQuery)]
                struct BodyRef<'a> {
                    #[query(storage = ".body.collider")]
                    position: &'a Position,
                    #[query(storage = ".body")]
                    velocity: &'a mut vec2<Coord>,
                }

                let mut actor_query = query_body_ref!(self.actors);
                let mut proj_query = query_body_ref!(self.projectiles);

                let process = |query: &mut Query<BodyRefQuery<'_>, ecs::arena::ArenaFamily>| {
                    let mut iter = query.iter_mut();
                    while let Some((_, body)) = iter.next() {
                        let delta = position.direction(*body.position, self.config.world_size);
                        let dist = delta.len();
                        if dist > radius {
                            continue;
                        }

                        let t = (Coord::ONE
                            - ((dist - Coord::ONE).max(Coord::ZERO) / radius).sqrt())
                        .clamp(Coord::ZERO, Coord::ONE);
                        let strength = strength * t;
                        let dir = delta.normalize_or_zero();
                        *body.velocity += dir * strength;
                    }
                };

                process(&mut actor_query);
                process(&mut proj_query);

                // TODO: account for distance
                let player = actor_query
                    .get(self.player.actor)
                    .expect("Player actor not found");
                let player_dist = player
                    .position
                    .direction(position, self.config.world_size)
                    .len()
                    .max(r32(0.1));
                let amplitude = (r32(30.0) / player_dist).clamp_range(r32(0.0)..=r32(100.0));
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
