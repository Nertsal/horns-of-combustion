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

        // Zoom out if player is moving fast.
        // let player_velocity = self.bodies.get(self.player.body).unwrap().velocity;
        // let player_speed = player_velocity.len();
        // camera.fov = TODO: interpolate fov to player speed.

        // Do not follow player if it is inside the bounds of the camera.
        let direction = *player_position - camera.center.as_r32();
        let distance = direction.len();
        if distance > camera.fov / r32(3.0) {
            self.player.out_of_view = true;
        }

        if self.player.out_of_view {
            let config = &self.config.camera;
            if distance < config.dead_zone {
                self.player.out_of_view = false;
                // camera.target_position = *player_position;
            } else {
                // Update the target position
                camera.target_position = *player_position;
            }

            // Interpolate camera position to the target
            // Take min to not overshoot the target
            camera.center += direction * (config.speed * delta_time).min(Coord::ONE);
        }
    }

    fn control_player(&mut self, delta_time: Time) {
        // Change target velocity to body velocity.
        let config = &self.config.player;
        let player = &mut self.player;
        let player_body = self.bodies.get_mut(player.body).unwrap();

        // Use player direction to change target velocity.
        player.target_velocity = player.player_direction * config.speed;

        // Interpolate body velocity to target velocity.
        *player_body.velocity +=
            (player.target_velocity - *player_body.velocity) * config.acceleration * delta_time;
    }

    /// System that moves all bodies in the world according to their velocity.
    fn movement(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct BodyRef<'a> {
            #[query(storage = ".collider")]
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
