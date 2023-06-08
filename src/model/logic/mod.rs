use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.movement(delta_time);
    }

    /// System that moves all bodies in the world according to their velocity.
    fn movement(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            #[query(nested = ".collider")]
            position: &'a mut vec2<Coord>,
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_body_ref!(self.bodies);
        let mut iter = query.iter_mut();
        while let Some((_id, body)) = iter.next() {
            // TODO: Move body
        }
    }
}
