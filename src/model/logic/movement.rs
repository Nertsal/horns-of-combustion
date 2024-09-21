use super::*;

impl Model {
    /// System that moves all bodies in the world according to their velocity.
    pub(super) fn movement(&mut self, delta_time: Time) {
        for (position, &velocity) in query!(
            [self.actors, self.projectiles, self.pickups],
            (&mut body.collider.position, &body.velocity,)
        ) {
            position.shift(velocity * delta_time);
        }
    }
}
