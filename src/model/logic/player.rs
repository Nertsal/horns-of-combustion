use super::*;

impl Model {
    pub(super) fn control_player(&mut self, delta_time: Time) {
        match self.player.state {
            PlayerState::Human => self.human_control(delta_time),
            PlayerState::Barrel { next_gas } => self.barrel_control(next_gas, delta_time),
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

    fn barrel_control(&mut self, mut next_gas: Coord, delta_time: Time) {
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
            #[query(storage = ".body.collider")]
            position: &'a vec2<Coord>,
            stats: &'a Stats,
        }

        let mut query = query_player_ref!(self.actors);
        let player = query
            .get_mut(self.player.actor)
            .expect("Player actor not found");

        // Update shape
        *player.shape = self.config.player.barrel_state.shape;

        // Controller
        // let input_direction = (self.player.aim_at - *player.position).normalize_or_zero();
        let input_direction = self.player.input_direction;
        let delta_angle = if input_direction == vec2::ZERO {
            Coord::ZERO
        } else {
            let current_angle = Angle::from_radians(player.velocity.arg());
            let target_angle = Angle::from_radians(input_direction.arg());
            let delta_angle = current_angle.angle_to(target_angle).as_radians();
            let steering = self.config.player.barrel_state.steering;
            delta_angle.clamp_abs(steering * delta_time)
        };
        player.controller.target_velocity = player.velocity.rotate(delta_angle);
        player.controller.acceleration = r32(100.0);

        // Look in the direction of travel
        *player.rotation = Angle::from_radians(player.velocity.arg());

        // Update state
        next_gas -= player.velocity.len() * delta_time;
        if next_gas <= Coord::ZERO {
            let config = &self.config.player.barrel_state.gasoline;
            next_gas = config.distance_period;
            self.gasoline.insert(Gasoline {
                collider: Collider::new(*player.position, config.shape),
                lifetime: Lifetime::new(config.lifetime),
                ignite_timer: config.ignite_timer,
                explosion_radius: config.explosion_radius,
                explosion_strength: config.explosion_strength,
                fire: config.fire,
            });
        }
        self.player.state = PlayerState::Barrel { next_gas };
    }
}
