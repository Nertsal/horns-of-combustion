use super::*;

impl Model {
    /// System that moves all bodies in the world according to their velocity.
    pub(super) fn movement(&mut self, delta_time: Time) {
        self.player.body.collider.position += self.player.body.velocity * delta_time;
        self.move_actors(delta_time);
        self.move_projectiles(delta_time);
    }

    fn move_actors(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a mut vec2<Coord>,
            #[query(storage = ".body")]
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_id, actor)) = iter.next() {
            // Move with constant velocity.
            *actor.position += *actor.velocity * delta_time;
        }
    }

    fn move_projectiles(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a mut vec2<Coord>,
            #[query(storage = ".body")]
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_proj_ref!(self.projectiles);
        let mut iter = query.iter_mut();
        while let Some((_id, body)) = iter.next() {
            // Move with constant velocity.
            *body.position += *body.velocity * delta_time;
        }
    }
}
