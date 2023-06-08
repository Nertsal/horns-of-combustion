use super::*;

impl Model {
    pub(super) fn actors_ai(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
            stats: &'a Stats,
            controller: &'a mut Controller,
            #[query(optic = "._Some")]
            ai: &'a mut ActorAI,
        }

        let player = &self.player;

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            let player_dir = player.actor.body.collider.position - *actor.position;
            // let player_dist = player_dir.len();
            let player_dir = player_dir.normalize_or_zero();

            match actor.ai {
                ActorAI::Crawler => {
                    actor.controller.target_velocity = player_dir * actor.stats.move_speed;
                }
            }
        }
    }

    pub(super) fn control_actors(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            controller: &'a Controller,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            // Interpolate body velocity to target velocity.
            *actor.velocity += (actor.controller.target_velocity - *actor.velocity)
                * actor.controller.acceleration
                * delta_time;
        }
    }
}
