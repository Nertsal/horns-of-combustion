use super::*;

impl Model {
    pub(super) fn control_projectiles(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            lifetime: &'a mut Lifetime,
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            target_pos: &'a Option<vec2<Coord>>,
            ai: &'a ProjectileAI,
        }

        let mut grounded_projs: Vec<Id> = Vec::new();
        let mut kill_projs: Vec<Id> = Vec::new();

        let mut query = query_proj_ref!(self.projectiles);
        let mut iter = query.iter_mut();
        while let Some((proj_id, proj)) = iter.next() {
            // Update lifetime
            proj.lifetime.damage(delta_time);
            if proj.lifetime.is_dead() {
                kill_projs.push(proj_id);
                continue;
            }

            if let Some(target_pos) = *proj.target_pos {
                // Target position is specified, so the projectile should stop at the target
                let target_dir = target_pos - *proj.position;
                if vec2::dot(target_dir, *proj.velocity) < Coord::ZERO {
                    // The projectile is travelling away from the target
                    grounded_projs.push(proj_id);
                }
            }

            match proj.ai {
                ProjectileAI::Straight => {}
                ProjectileAI::ConstantTurn { degrees_per_second } => {
                    // Change velocity direction by a constant angle
                    let angle = Angle::from_degrees(*degrees_per_second * delta_time);
                    *proj.velocity = proj.velocity.rotate(angle.as_radians());
                }
            }
        }

        for id in kill_projs {
            // TODO: smoothly
            self.projectiles.remove(id);
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
        }

        let gas_query = query_gas_ref!(self.gasoline);

        // Every grounded projectile ignites gasoline
        let mut ignite: Vec<Id> = Vec::new();
        for proj_id in grounded_projs {
            let proj = self.projectiles.remove(proj_id).unwrap();
            for (gas_id, gas) in &gas_query {
                if proj.body.collider.check(gas.collider) {
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
