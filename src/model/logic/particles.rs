use super::*;

impl Model {
    pub(super) fn update_particles(&mut self, delta_time: Time) {
        struct ParticleRef<'a> {
            position: &'a mut Position,
            velocity: &'a mut vec2<Coord>,
            lifetime: &'a mut Lifetime,
            kind: &'a ParticleKind,
        }

        let mut to_remove: Vec<Id> = Vec::new();
        for id in self.particles.ids() {
            let particle = get!(
                self.particles,
                id,
                ParticleRef {
                    position: &mut position,
                    velocity: &mut velocity,
                    lifetime: &mut lifetime,
                    kind,
                }
            );
            let Some(particle) = particle else { continue };

            // Update lifetime
            particle.lifetime.change(-delta_time);
            if particle.lifetime.is_min() {
                to_remove.push(id);
                continue;
            }

            // Control
            match particle.kind {
                ParticleKind::Fire => {
                    let amplitude = particle.lifetime.get_ratio();
                    let t = particle.lifetime.value().sin();
                    let angle = Angle::from_radians(t * amplitude);
                    let dir = angle.unit_vec().rotate_90();
                    *particle.velocity = dir * particle.velocity.len();
                }
                ParticleKind::Damage | ParticleKind::Heal | ParticleKind::Projectile => {}
            }

            // Move
            particle.position.shift(*particle.velocity * delta_time);
        }

        for id in to_remove {
            self.particles.remove(id);
        }
    }
}
