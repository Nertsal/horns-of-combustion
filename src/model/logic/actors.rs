use super::*;

impl Model {
    pub(super) fn actors_ai(&mut self, _delta_time: Time) {
        struct ActorRef<'a> {
            position: &'a mut Position,
            rotation: &'a mut Angle<Coord>,
            velocity: &'a mut vec2<Coord>,
            stats: &'a Stats,
            controller: &'a mut Controller,
            ai: &'a mut ActorAI,
            kind: &'a mut ActorKind,
            gun: &'a mut Option<Gun>,
            stunned: &'a Option<Time>,
        }

        let Some(player) = self.actors.get(self.player.actor) else {
            return;
        };
        let player = player.clone();

        let mut shots = Vec::new();

        for actor_id in self.actors.ids() {
            let actor = get!(
                self.actors,
                actor_id,
                ActorRef {
                    position: &mut body.collider.position,
                    rotation: &mut body.collider.rotation,
                    velocity: &mut body.velocity,
                    stats,
                    controller: &mut controller,
                    ai: &mut ai.Get.Some,
                    kind: &mut kind,
                    gun: &mut gun,
                    stunned,
                }
            );
            let Some(actor) = actor else { continue };

            if actor.stunned.is_some() {
                continue;
            }

            let player_dir = actor.position.delta_to(player.body.collider.position);
            // let player_dist = player_dir.len();
            let player_dir = player_dir.normalize_or_zero();

            match actor.ai {
                ActorAI::Crawler => {
                    actor.controller.target_velocity = player_dir * actor.stats.move_speed;
                }
                ActorAI::Ranger { preferred_distance } => {
                    let target = player
                        .body
                        .collider
                        .position
                        .shifted(-player_dir * *preferred_distance);
                    let target_dir = actor.position.delta_to(target).normalize_or_zero();
                    actor.controller.target_velocity = target_dir * actor.stats.move_speed;

                    if let ActorKind::EnemyDeathStar = actor.kind {
                        *actor.rotation += Angle::from_degrees(
                            actor.velocity.len() * actor.velocity.x.signum() / r32(4.0),
                        );
                    }

                    if let Some(gun) = actor.gun {
                        if gun.shot_delay <= Time::ZERO {
                            gun.shot_delay = gun.config.shot_delay;
                            let target_pos = player.body.collider.position;
                            let dir = actor.position.delta_to(target_pos);
                            *actor.velocity -= dir.normalize_or_zero() * gun.config.recoil;
                            shots.push((
                                *actor.position,
                                target_pos,
                                Fraction::Enemy,
                                gun.config.shot.clone(),
                            ));
                        }
                    }
                }
                ActorAI::BossFoot { position } => {
                    *actor.velocity = vec2::ZERO;

                    let sign = position.as_dir().x.signum();
                    let rotation = r32((self.time.as_f32() * 3.0).sin() * 0.8 - 0.2) * sign;
                    let point = vec2(-7.0 * -sign.as_f32(), -3.0).as_r32();

                    *actor.position = position.shifted(point.rotate(rotation));
                    // *actor.position = Position::from_world(point, self.config.world_size);
                    *actor.rotation = Angle::from_radians(rotation);

                    if actor.rotation.as_radians().abs() > r32(0.99) {
                        let target_pos = player.body.collider.position;
                        shots.push((
                            *actor.position,
                            target_pos,
                            Fraction::Enemy,
                            ShotConfig {
                                pattern: ShotPattern::Multiple {
                                    spread_degrees: r32(270.0),
                                    bullets: 9,
                                },
                                projectile: ProjectileConfig {
                                    lifetime: r32(5.0),
                                    speed: r32(25.0),
                                    damage: r32(15.0),
                                    body: BodyConfig {
                                        shape: Shape::Circle { radius: r32(0.2) },
                                        mass: R32::ONE,
                                    },
                                    ai: ProjectileAI::ConstantTurn {
                                        degrees_per_second: r32(90.0),
                                    },
                                    kind: ProjectileKind::SquidLike,
                                    knockback: r32(10.0),
                                },
                            },
                        ))
                    }
                }
                ActorAI::BossBody => {
                    *actor.velocity = vec2::ZERO;
                }
            }
        }

        for (pos, aimed_towards, fraction, config) in shots {
            self.shoot(pos, aimed_towards, fraction, config);
        }
    }

    pub(super) fn control_actors(&mut self, delta_time: Time) {
        struct ActorRef<'a> {
            velocity: &'a mut vec2<Coord>,
            controller: &'a Controller,
            stunned: &'a mut Option<Time>,
        }

        for id in self.actors.ids() {
            let actor = get!(
                self.actors,
                id,
                ActorRef {
                    velocity: &mut body.velocity,
                    controller,
                    stunned: &mut stunned,
                }
            );
            let Some(actor) = actor else { continue };

            let target_velocity = if let Some(time) = actor.stunned {
                *time -= delta_time;
                if *time <= Time::ZERO {
                    *actor.stunned = None;
                }
                vec2::ZERO
            } else {
                actor.controller.target_velocity
            };

            // Interpolate body velocity to target velocity.
            // Take min(1.0) to not overshoot
            *actor.velocity += (target_velocity - *actor.velocity)
                * (actor.controller.acceleration * delta_time).min(Coord::ONE);
        }
    }
}
