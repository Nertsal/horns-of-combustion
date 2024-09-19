use super::*;

impl Model {
    /// System that moves all bodies in the world according to their velocity.
    pub(super) fn movement(&mut self, delta_time: Time) {
        struct MoveRef<'a> {
            position: &'a mut Position,
            velocity: &'a vec2<Coord>,
        }

        let process = |body: MoveRef<'_>| {
            body.position.shift(*body.velocity * delta_time);
        };

        // TODO: global query
        let actors = query!(
            self.actors,
            MoveRef {
                position: &mut body.collider.position,
                velocity: &body.velocity,
            }
        );
        let projectiles = query!(
            self.projectiles,
            MoveRef {
                position: &mut body.collider.position,
                velocity: &body.velocity,
            }
        );
        let pickups = query!(
            self.pickups,
            MoveRef {
                position: &mut body.collider.position,
                velocity: &body.velocity,
            }
        );
        for (_id, body) in actors.chain(projectiles).chain(pickups) {
            process(body);
        }
    }
}
