use super::*;

impl Model {
    pub(super) fn control_player(&mut self, delta_time: Time) {
        match self.player.state {
            PlayerState::Human => self.human_control(delta_time),
            PlayerState::Barrel { last_gas } => self.barrel_control(last_gas, delta_time),
        };
    }

    fn human_control(&mut self, _delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct PlayerRef<'a> {
            // #[query(storage = ".body")]
            // velocity: &'a mut vec2<Coord>,
            #[query(storage = ".body.collider")]
            rotation: &'a mut Angle<Coord>,
            #[query(storage = ".body.collider")]
            shape: &'a mut Shape,
            controller: &'a mut Controller,
            stats: &'a Stats,
        }

        let mut query = query_player_ref!(self.actors);
        let Some(player) = query.get_mut(self.player.actor) else {
            return;
        };

        // Reset rotation
        *player.rotation = Angle::ZERO;

        // Update shape
        *player.shape = self.config.player.human_state.shape;

        // Controller
        player.controller.target_velocity = self.player.input.direction * player.stats.move_speed;
        player.controller.acceleration = self.config.player.acceleration;
    }

    fn barrel_control(&mut self, mut last_gas: Position, delta_time: Time) {
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
            position: &'a Position,
            stats: &'a Stats,
        }

        let mut query = query_player_ref!(self.actors);
        let Some(player) = query.get_mut(self.player.actor) else {
            return;
        };

        // Update shape
        *player.shape = self.config.player.barrel_state.shape;

        // Controller
        // let input_direction = (self.player.aim_at - *player.position).normalize_or_zero();
        let input_direction = self.player.input.direction;
        let delta_angle = if input_direction == vec2::ZERO {
            Coord::ZERO
        } else {
            let current_angle = Angle::from_radians(player.velocity.arg());
            let target_angle = Angle::from_radians(input_direction.arg());
            let delta_angle = current_angle.angle_to(target_angle).as_radians();
            let steering = self.config.player.barrel_state.steering;
            delta_angle.clamp_abs(steering * delta_time)
        };
        player.controller.target_velocity = player
            .velocity
            .rotate(delta_angle)
            .clamp_len(..=self.config.player.barrel_state.speed);
        player.controller.acceleration = r32(100.0);

        // Look in the direction of travel
        *player.rotation = Angle::from_radians(player.velocity.arg());

        // Drip gasoline
        let config = &self.config.player.barrel_state.gasoline;
        if !config.can_control || self.player.input.drip_gas {
            let pos = *player.position;
            let last_delta = pos.direction(last_gas, self.config.world_size);
            let last_dir = last_delta.normalize_or_zero();
            let mut last_dist = last_delta.len();
            while last_dist >= config.distance_period {
                let position =
                    last_gas.shifted(-last_dir * config.distance_period, self.config.world_size);
                last_gas = position;
                last_dist -= config.distance_period;

                if self.player.gasoline.hp < config.cost {
                    break;
                }
                self.player.gasoline.damage(config.cost);
                self.gasoline.insert(Gasoline {
                    collider: Collider::new(position, config.shape),
                    lifetime: Lifetime::new(config.lifetime),
                    ignite_timer: config.ignite_timer,
                    fire_radius: config.fire_radius,
                    explosion: config.explosion.clone(),
                    fire: config.fire.clone(),
                });
            }
        } else {
            last_gas = *player.position;
        }

        self.player.state = PlayerState::Barrel { last_gas };
    }
}
