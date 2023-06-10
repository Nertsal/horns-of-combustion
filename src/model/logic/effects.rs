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
            Effect::Explosion {
                position,
                radius,
                strength,
            } => {
                // TODO: visual

                #[allow(dead_code)]
                #[derive(StructQuery)]
                struct BodyRef<'a> {
                    #[query(storage = ".body.collider")]
                    position: &'a vec2<Coord>,
                    #[query(storage = ".body")]
                    velocity: &'a mut vec2<Coord>,
                }

                let actor_query = query_body_ref!(self.actors);
                let proj_query = query_body_ref!(self.projectiles);

                let process = |mut query: Query<BodyRefQuery<'_>, ecs::arena::ArenaFamily>| {
                    let mut iter = query.iter_mut();
                    while let Some((_, body)) = iter.next() {
                        let delta = *body.position - position;
                        let dist = delta.len();
                        if dist > radius {
                            continue;
                        }

                        let dir = delta.normalize_or_zero();
                        *body.velocity += dir * strength;
                    }
                };

                process(actor_query);
                process(proj_query);
            }
        }
    }
}
