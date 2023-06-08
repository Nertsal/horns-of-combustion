use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.control_player(delta_time);
        self.movement(delta_time);
        self.update_camera(delta_time);
    }

    fn update_camera(&mut self, delta_time: Time) {
        let player_position = self.bodies.get(self.player.body).unwrap().collider.position;
        let camera = &mut self.camera;

        // TODO: Use camera zoom.
        // TODO: Use camera boundaries.

        // Interpolate camera position to player position.
        camera.center += (player_position.as_f32() - camera.center) * 0.1; // TODO: <--- move to camera config.
    }

    fn control_player(&mut self, delta_time: Time) {
        // Change target velocity to body velocity.
        let player = &mut self.player;
        let player_body = self.bodies.get_mut(player.body).unwrap();

        // Use player direction to change target velocity.
        player.target_velocity = player.player_direction * player.player_speed;

        // Interpolate body velocity to target velocity.
        *player_body.velocity += (player.target_velocity - *player_body.velocity) * player.player_acceleration;
    }

    /// System that moves all bodies in the world according to their velocity.
    fn movement(&mut self, delta_time: Time) {
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            #[query(optic = ".collider._get", component = "Collider")]
            position: &'a mut vec2<Coord>,
            velocity: &'a vec2<Coord>,
        }

        let mut query = query_body_ref!(self.bodies);
        let mut iter = query.iter_mut();
        while let Some((_id, body)) = iter.next() {
            // Move with constant velocity.
            *body.position += *body.velocity * delta_time;
        }
    }
}
