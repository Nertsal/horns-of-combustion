use super::*;

impl Model {
    pub(super) fn control_player(&mut self, delta_time: Time) {
        match self.player.state {
            PlayerState::Human => self.human_control(delta_time),
            PlayerState::Barrel => self.barrel_control(delta_time),
        };
    }

    fn human_control(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            // #[query(storage = ".body")]
            // velocity: &'a mut vec2<Coord>,
            #[query(storage = ".body.collider")]
            shape: &'a mut Shape,
            controller: &'a mut Controller,
            stats: &'a Stats,
        }

        let mut query = query_player_ref!(self.actors);
        let player = query
            .get_mut(self.player.actor)
            .expect("Player actor not found");

        // Update shape
        *player.shape = self.config.player.human_state.shape;

        // Controller
        player.controller.target_velocity = self.player.input_direction * player.stats.move_speed;
        player.controller.acceleration = self.config.player.acceleration;
    }

    fn barrel_control(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            #[query(storage = ".body")]
            velocity: &'a mut vec2<Coord>,
            #[query(storage = ".body.collider")]
            shape: &'a mut Shape,
            #[query(storage = ".body.collider")]
            rotation: &'a mut Angle<Coord>,
            controller: &'a mut Controller,
            stats: &'a Stats,
        }

        let mut query = query_player_ref!(self.actors);
        let player = query
            .get_mut(self.player.actor)
            .expect("Player actor not found");

        // Update shape
        *player.shape = self.config.player.barrel_state.shape;

        // Controller
        // left (-1) steers in the positive angle
        let steering = self.config.player.barrel_state.steering;
        let input_angle = -self.player.input_direction.x * steering;
        player.controller.target_velocity = player.velocity.rotate(input_angle * delta_time);
        player.controller.acceleration = r32(100.0);

        // Look in the direction of travel
        *player.rotation = Angle::from_radians(player.velocity.arg());
    }
}
