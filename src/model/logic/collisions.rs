use super::*;

impl Model {
    pub(super) fn collisions(&mut self, delta_time: Time) {
        self.collide_projectiles(delta_time);
    }

    fn collide_projectiles(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            damage: &'a Hp,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            health: &'a mut Health,
        }

        let mut proj_hits: Vec<Id> = Vec::new();
        let mut dead_actors: Vec<Id> = Vec::new();

        let mut actors = query_actor_ref!(self.actors);

        let mut projs = query_proj_ref!(self.projectiles);
        let mut proj_iter = projs.iter_mut();
        while let Some((proj_id, proj)) = proj_iter.next() {
            let mut actor_iter = actors.iter_mut();
            while let Some((actor_id, actor)) = actor_iter.next() {
                if proj.collider.clone().check(&actor.collider.clone()) {
                    proj_hits.push(proj_id);
                    actor.health.damage(*proj.damage);
                    if actor.health.is_dead() {
                        dead_actors.push(actor_id);
                    }
                    break;
                }
            }
        }

        for id in proj_hits {
            self.projectiles.remove(id);
        }
        for id in dead_actors {
            self.actors.remove(id);
        }
    }
}
