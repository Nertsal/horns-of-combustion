use super::*;

impl Model {
    pub(super) fn collisions(&mut self, delta_time: Time) {
        self.collide_projectiles(delta_time);
        self.fire_gas(delta_time);
    }

    fn collide_projectiles(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            fraction: &'a Fraction,
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            damage: &'a Hp,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            fraction: &'a Fraction,
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
                if proj.fraction == actor.fraction {
                    continue;
                }
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

    fn fire_gas(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            collider: &'a Collider,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
        }

        let fire_query = query_fire_ref!(self.fire);
        let gas_query = query_gas_ref!(self.gasoline);

        // TODO: fix framerate dependency
        // e.g. iterate until no more overlaps are found
        let mut to_ignite: Vec<Id> = Vec::new();
        for (gas_id, gas) in &gas_query {
            for (_, fire) in &fire_query {
                if fire.collider.check(gas.collider) {
                    to_ignite.push(gas_id);
                    break;
                }
            }
        }

        for gas_id in to_ignite {
            let gas = self.gasoline.remove(gas_id).unwrap();
            self.fire.insert(Fire {
                collider: gas.collider,
                lifetime: Health::new(5.0),
            });
        }
    }
}
