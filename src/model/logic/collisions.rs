use super::*;

impl Model {
    pub(super) fn collisions(&mut self, delta_time: Time) {
        self.collide_barrel(delta_time);
        self.collide_projectiles(delta_time);
        self.projectile_gas(delta_time);
        self.fire_gas(delta_time);
        self.fire(delta_time);
    }

    fn collide_barrel(&mut self, _delta_time: Time) {
        if !matches!(self.player.state, PlayerState::Barrel { .. }) {
            return;
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            #[query(storage = ".body")]
            velocity: &'a vec2<Coord>,
            stops_barrel: &'a bool,
        }

        let query = query_actor_ref!(self.actors);
        let player = query
            .get(self.player.actor)
            .expect("Player actor not found");
        let player_collider = &player.collider.clone();

        #[derive(Clone)]
        struct Correction {
            position: vec2<Coord>,
            velocity: vec2<Coord>,
            stun: Option<Time>,
        }
        let mut corrections: HashMap<Id, Correction> = HashMap::new();

        for (actor_id, actor) in &query {
            if actor_id == self.player.actor {
                continue;
            }
            if let Some(collision) = player_collider.collide(&actor.collider.clone()) {
                let mut player_cor =
                    corrections
                        .get(&self.player.actor)
                        .cloned()
                        .unwrap_or(Correction {
                            position: *player.collider.position,
                            velocity: *player.velocity,
                            stun: None,
                        });
                let mut actor_cor = corrections.get(&actor_id).cloned().unwrap_or(Correction {
                    position: *actor.collider.position,
                    velocity: *actor.velocity,
                    stun: None,
                });

                // TODO: configurable + better formula
                actor_cor.stun = if *actor.stops_barrel {
                    None
                } else {
                    Some(r32(3.0))
                };

                let relative_vel = player_cor.velocity - *actor.velocity;
                let dot = vec2::dot(relative_vel, collision.normal);
                if dot <= Coord::ZERO {
                    continue;
                }

                let hit_barrel = if *actor.stops_barrel {
                    dot
                } else {
                    dot.min(r32(2.0))
                };
                player_cor.velocity -= collision.normal * hit_barrel;

                corrections.insert(self.player.actor, player_cor);
                corrections.insert(actor_id, actor_cor);
            }
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct UpdateRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a mut vec2<Coord>,
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            stunned: &'a mut Option<Time>,
        }

        let mut query = query_update_ref!(self.actors);
        for (id, correction) in corrections {
            let actor = query.get_mut(id).expect("invalid correction");
            *actor.position = correction.position;
            *actor.velocity = correction.velocity;
            *actor.stunned = std::cmp::max(*actor.stunned, correction.stun);
        }
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

        let mut actors = query_actor_ref!(self.actors);

        let mut projs = query_proj_ref!(self.projectiles);
        let mut proj_iter = projs.iter_mut();
        while let Some((proj_id, proj)) = proj_iter.next() {
            let mut actor_iter = actors.iter_mut();
            while let Some((_actor_id, actor)) = actor_iter.next() {
                if proj.fraction == actor.fraction {
                    continue;
                }
                if proj.collider.clone().check(&actor.collider.clone()) {
                    proj_hits.push(proj_id);
                    actor.health.damage(*proj.damage);
                    break;
                }
            }
        }

        for id in proj_hits {
            self.projectiles.remove(id);
        }
    }

    /// Projectiles ignite gas when passing over it.
    fn projectile_gas(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
        }

        let mut gas_ignited: Vec<Id> = Vec::new();

        for (gas_id, gas) in &query_gas_ref!(self.gasoline) {
            for (_proj_id, proj) in &query_proj_ref!(self.projectiles) {
                if proj.collider.clone().check(gas.collider) {
                    gas_ignited.push(gas_id);
                    break;
                }
            }
        }

        for id in gas_ignited {
            self.ignite_gasoline(id);
        }
    }

    fn fire_gas(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            collider: &'a Collider,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
            ignite_timer: &'a mut Time,
        }

        let fire_query = query_fire_ref!(self.fire);
        let mut gas_query = query_gas_ref!(self.gasoline);

        let mut gas_iter = gas_query.iter_mut();
        let mut to_ignite: Vec<Id> = Vec::new();

        while let Some((gas_id, gas)) = gas_iter.next() {
            for (_, fire) in &fire_query {
                if fire.collider.check(gas.collider) {
                    *gas.ignite_timer -= delta_time;
                    if *gas.ignite_timer <= Time::ZERO {
                        to_ignite.push(gas_id);
                        break;
                    }
                }
            }
        }

        for gas_id in to_ignite {
            self.ignite_gasoline(gas_id);
        }
    }

    fn fire(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            collider: &'a Collider,
            config: &'a FireConfig,
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(nested, storage = ".body")]
            collider: &'a Collider,
            on_fire: &'a mut Option<OnFire>,
        }

        let fire_query = query_fire_ref!(self.fire);

        let mut actors_query = query_actor_ref!(self.actors);
        let mut actors_iter = actors_query.iter_mut();
        while let Some((_, actor)) = actors_iter.next() {
            for (_, fire) in &fire_query {
                if actor.collider.clone().check(fire.collider) {
                    let mut on_fire = actor.on_fire.clone().unwrap_or(OnFire {
                        duration: Time::ZERO,
                        damage_per_second: Hp::ZERO,
                    });
                    on_fire.duration = on_fire.duration.max(fire.config.duration);
                    on_fire.damage_per_second =
                        on_fire.damage_per_second.max(fire.config.damage_per_second);
                    *actor.on_fire = Some(on_fire);
                }
            }
        }
    }
}