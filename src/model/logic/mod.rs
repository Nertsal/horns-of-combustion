mod action;
mod actors;
mod collisions;
mod movement;
mod player;
mod weapons;

use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.update_weapons(delta_time);
        self.actors_ai(delta_time);
        self.control_player(delta_time);
        self.control_actors(delta_time);
        self.movement(delta_time);
        self.collisions(delta_time);
        self.update_camera(delta_time);
    }

    fn update_camera(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
        }

        let query = query_player_ref!(self.actors);
        let player = query
            .get(self.player.actor)
            .expect("Player actor not found");

        let camera = &mut self.camera;

        // Zoom out if player is moving fast.
        // let player_velocity = self.bodies.get(self.player.body).unwrap().velocity;
        // let player_speed = player_velocity.len();
        // camera.fov = TODO: interpolate fov to player speed.

        // Do not follow player if it is inside the bounds of the camera.
        let direction = *player.position - camera.center.as_r32();
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
                camera.target_position = *player.position;
            }

            // Interpolate camera position to the target
            // Take min to not overshoot the target
            camera.center += direction * (config.speed * delta_time).min(Coord::ONE);
        }
    }
}
