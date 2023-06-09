use super::*;

impl Model {
    pub(super) fn actors_ai(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
            stats: &'a Stats,
            controller: &'a mut Controller,
            #[query(optic = "._Some")]
            ai: &'a mut ActorAI,
            gun: &'a mut Option<Gun>,
        }

        let player = self
            .actors
            .get(self.player.actor)
            .expect("Player actor not found")
            .clone();

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            let player_dir = player.body.collider.position - *actor.position;
            // let player_dist = player_dir.len();
            let player_dir = player_dir.normalize_or_zero();

            match actor.ai {
                ActorAI::Crawler => {
                    actor.controller.target_velocity = player_dir * actor.stats.move_speed;
                }
                ActorAI::Ranger { preferred_distance } => {
                    let target = player.body.collider.position - player_dir * *preferred_distance;
                    let target_dir = (target - *actor.position).normalize_or_zero();
                    actor.controller.target_velocity = target_dir * actor.stats.move_speed;

                    if let Some(gun) = actor.gun {
                        if gun.shot_delay <= Time::ZERO {
                            gun.shot_delay = gun.config.shot_delay;
                            self.projectiles.insert(Projectile::new(
                                *actor.position,
                                player.body.collider.position,
                                Fraction::Enemy,
                                gun.config.projectile,
                            ));
                        }
                    }
                }
            }
        }
    }

    pub(super) fn control_actors(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            controller: &'a Controller,
            stunned: &'a mut Option<Time>,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
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
