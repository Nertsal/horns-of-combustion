use super::*;

impl Model {
    pub(super) fn update_particles(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ParticleRef<'a> {
            position: &'a mut Position,
            velocity: &'a mut vec2<Coord>,
            lifetime: &'a mut Lifetime,
            kind: &'a ParticleKind,
        }

        let mut query = query_particle_ref!(self.particles);

        let mut iter = query.iter_mut();
        let mut to_remove: Vec<Id> = Vec::new();
        while let Some((id, particle)) = iter.next() {
            // Update lifetime
            particle.lifetime.damage(delta_time);
            if particle.lifetime.is_dead() {
                to_remove.push(id);
                continue;
            }

            // Control
            match particle.kind {
                ParticleKind::Fire => {
                    let amplitude = particle.lifetime.ratio();
                    let t = particle.lifetime.hp.sin();
                    *particle.velocity =
                        vec2::UNIT_Y.rotate(t * amplitude) * particle.velocity.len();
                }
            }

            // Move
            particle
                .position
                .shift(*particle.velocity * delta_time, self.config.world_size);
        }

        for id in to_remove {
            self.particles.remove(id);
        }
    }
}
