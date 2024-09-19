use super::*;

impl Model {
    pub(super) fn control_projectiles(&mut self, delta_time: Time) {
        struct ProjRef<'a> {
            id: Index,
            lifetime: &'a mut Lifetime,
            fraction: &'a Fraction,
            position: &'a Position,
            rotation: &'a mut Angle<R32>,
            velocity: &'a mut vec2<Coord>,
            target_pos: &'a Option<Position>,
            ai: &'a mut ProjectileAI,
        }

        let mut grounded_projs: Vec<Id> = Vec::new();
        let mut kill_projs: Vec<Id> = Vec::new();
        let mut to_be_spawned: Vec<Projectile> = Vec::new();

        for proj in query!(
            self.projectiles,
            ProjRef {
                id,
                lifetime: &mut lifetime,
                fraction,
                position: &body.collider.position,
                rotation: &mut body.collider.rotation,
                velocity: &mut body.velocity,
                target_pos,
                ai: &mut ai,
            }
        ) {
            // Update lifetime
            proj.lifetime.change(-delta_time);
            if proj.lifetime.is_min() {
                kill_projs.push(proj.id);
                continue;
            }

            // Update rotation
            *proj.rotation = proj.velocity.arg();

            if let Some(target_pos) = *proj.target_pos {
                // Target position is specified, so the projectile should stop at the target
                let target_dir = proj.position.delta_to(target_pos);
                if vec2::dot(target_dir, *proj.velocity) < Coord::ZERO {
                    // The projectile is travelling away from the target
                    grounded_projs.push(proj.id);
                }
            }

            match proj.ai {
                ProjectileAI::Straight => {}
                ProjectileAI::ConstantTurn { degrees_per_second } => {
                    // Change velocity direction by a constant angle
                    let angle = Angle::from_degrees(*degrees_per_second * delta_time);
                    *proj.velocity = proj.velocity.rotate(angle);
                }
                ProjectileAI::CircleBomb {
                    explosive_type,
                    delay,
                } => {
                    // Until the delay is over, the projectile flies straight
                    if *delay >= Time::ZERO {
                        *delay -= delta_time;
                    } else {
                        // Explode!
                        kill_projs.push(proj.id);

                        // Create a circle of projectiles
                        for i in 0..18 {
                            to_be_spawned.push(Projectile::new(
                                *proj.position,
                                Angle::from_degrees(r32(i as f32 * 20.0)),
                                *proj.fraction,
                                *explosive_type.clone(),
                            ));
                        }
                    }
                }
            }
        }

        for id in kill_projs {
            // TODO: smoothly
            self.projectiles.remove(id);
        }

        for proj in to_be_spawned {
            self.projectiles.insert(proj);
        }

        // Every grounded projectile ignites gasoline
        let mut ignite: Vec<Id> = Vec::new();
        for proj_id in grounded_projs {
            let proj = self.projectiles.remove(proj_id).unwrap();
            for (gas_id, gas_collider) in query!(self.gasoline, (id, &collider)) {
                if proj.body.collider.check(&gas_collider.clone()) {
                    // Ignite gasoline
                    ignite.push(gas_id);
                }
            }
        }

        for id in ignite {
            self.ignite_gasoline(id);
        }
    }
}
