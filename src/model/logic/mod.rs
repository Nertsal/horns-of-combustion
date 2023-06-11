mod action;
mod actors;
mod collisions;
mod effects;
mod movement;
mod player;
mod projectiles;
mod waves;
mod weapons;

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.time += delta_time;

        self.update_weapons(delta_time);
        self.update_gas(delta_time);
        self.update_fire(delta_time);
        self.update_on_fire(delta_time);
        self.update_waves(delta_time);
        self.actors_ai(delta_time);
        self.control_player(delta_time);
        self.control_actors(delta_time);
        self.control_projectiles(delta_time);
        self.movement(delta_time);
        self.collisions(delta_time);
        self.handle_effects(delta_time);
        self.check_deaths(delta_time);
        self.update_camera(delta_time);
    }

    fn check_deaths(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            health: &'a Health,
        }

        let dead_actors: Vec<Id> = query_actor_ref!(self.actors)
            .iter()
            .filter(|(_, actor)| actor.health.is_dead())
            .map(|(id, _)| id)
            .collect();
        for id in dead_actors {
            let actor = self.actors.remove(id).unwrap();
            self.queued_effects.push_back(QueuedEffect {
                effect: Effect::Explosion {
                    position: actor.body.collider.position,
                    radius: self.config.death_explosion_radius,
                    strength: self.config.death_explosion_strength,
                },
            });
        }
    }

    fn ignite_gasoline(&mut self, gas_id: Id) {
        if let Some(gas) = self.gasoline.remove(gas_id) {
            self.queued_effects.push_back(QueuedEffect {
                effect: Effect::Explosion {
                    position: gas.collider.position,
                    radius: gas.explosion_radius,
                    strength: gas.explosion_strength,
                },
            });
            self.fire.insert(Fire {
                collider: Collider::new(
                    gas.collider.position,
                    Shape::Circle {
                        radius: gas.explosion_radius,
                    },
                ),
                lifetime: Lifetime::new(5.0),
                config: gas.fire,
            });
        }
    }

    fn update_camera(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
        }

        let query = query_player_ref!(self.actors);
        let player = query
            .get(self.player.actor)
            .expect("Player actor not found");

        let camera = &mut self.camera;

        // Zoom out if player is moving fast.
        // let player_velocity = self.bodies.get(self.player.body).unwrap().velocity;
        // let player_speed = player_velocity.len();
        // camera.fov = TODO: interpolate fov to player speed.

        // Do not follow player if it is inside the bounds of the camera.
        let direction = *player.position - camera.center.as_r32();
        let distance = direction.len();
        if distance > camera.fov / r32(3.0) {
            self.player.out_of_view = true;
        }

        if self.player.out_of_view {
            let config = &self.config.camera;
            if distance < config.dead_zone {
                self.player.out_of_view = false;
                // camera.target_position = *player_position;
            } else {
                // Update the target position
                camera.target_position = *player.position;
            }

            // Interpolate camera position to the target
            // Take min to not overshoot the target
            camera.center += direction * (config.speed * delta_time).min(Coord::ONE);
        }
    }

    fn update_gas(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut query = query_gas_ref!(self.gasoline);
        let mut iter = query.iter_mut();
        let mut expired: Vec<Id> = Vec::new();
        while let Some((id, gas)) = iter.next() {
            gas.lifetime.damage(delta_time);
            if gas.lifetime.is_dead() {
                expired.push(id);
            }
        }

        for id in expired {
            self.gasoline.remove(id);
        }
    }

    fn update_fire(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct FireRef<'a> {
            lifetime: &'a mut Lifetime,
        }

        let mut query = query_fire_ref!(self.fire);
        let mut iter = query.iter_mut();
        let mut expired: Vec<Id> = Vec::new();
        while let Some((id, fire)) = iter.next() {
            fire.lifetime.damage(delta_time);
            if fire.lifetime.is_dead() {
                expired.push(id);
            }
        }

        for id in expired {
            self.fire.remove(id);
        }
    }

    fn update_on_fire(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            health: &'a mut Health,
            on_fire: &'a mut Option<OnFire>,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_, actor)) = iter.next() {
            if let Some(on_fire) = actor.on_fire {
                actor.health.damage(on_fire.damage_per_second * delta_time);
                on_fire.duration -= delta_time;
                if on_fire.duration <= Time::ZERO {
                    *actor.on_fire = None;
                }
            }
        }
    }
}
