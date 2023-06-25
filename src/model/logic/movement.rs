use super::*;

impl Model {
    /// System that moves all bodies in the world according to their velocity.
    pub(super) fn movement(&mut self, delta_time: Time) {
        struct MoveRef<'a> {
            position: &'a mut Position,
            velocity: &'a vec2<Coord>,
        }

        let process = |body: Option<MoveRef<'_>>| {
            if let Some(body) = body {
                body.position
                    .shift(*body.velocity * delta_time, self.config.world_size);
            }
        };

        for id in self.actors.ids() {
            process(get!(
                self.actors,
                id,
                MoveRef {
                    position: &mut body.collider.position,
                    velocity: &body.velocity,
                }
            ));
        }
        for id in self.projectiles.ids() {
            process(get!(
                self.projectiles,
                id,
                MoveRef {
                    position: &mut body.collider.position,
                    velocity: &body.velocity,
                }
            ));
        }
        for id in self.pickups.ids() {
            process(get!(
                self.pickups,
                id,
                MoveRef {
                    position: &mut body.collider.position,
                    velocity: &body.velocity,
                }
            ));
        }
    }
}
