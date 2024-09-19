use super::*;

impl Model {
    /// System that moves all bodies in the world according to their velocity.
    pub(super) fn movement(&mut self, delta_time: Time) {
        struct MoveRef<'a> {
            position: &'a mut Position,
            velocity: &'a vec2<Coord>,
        }

        for body in query!(
            [self.actors, self.projectiles, self.pickups],
            MoveRef {
                position: &mut body.collider.position,
                velocity: &body.velocity,
            }
        ) {
            body.position.shift(*body.velocity * delta_time);
        }
    }
}
