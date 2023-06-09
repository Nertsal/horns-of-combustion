use super::*;

impl Model {
    pub(super) fn control_projectiles(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ProjRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
            #[query(storage = ".body")]
            velocity: &'a vec2<Coord>,
            #[query(optic = "._Some")]
            target_pos: &'a vec2<Coord>,
        }

        let mut grounded_projs: Vec<Id> = Vec::new();

        let mut query = query_proj_ref!(self.projectiles);
        let mut iter = query.iter_mut();
        while let Some((proj_id, proj)) = iter.next() {
            let target_dir = *proj.target_pos - *proj.position;
            if vec2::dot(target_dir, *proj.velocity) < Coord::ZERO {
                // The projectile is travelling away from the target
                grounded_projs.push(proj_id);
            }
        }

        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct GasRef<'a> {
            collider: &'a Collider,
        }

        let gas_query = query_gas_ref!(self.gasoline);

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
            // Multiple projectiles might ignite the same gasoline
            if let Some(gas) = self.gasoline.remove(id) {
                self.fire.insert(Fire {
                    collider: gas.collider,
                    lifetime: Lifetime::new(5.0),
                });
            }
        }
    }
}
