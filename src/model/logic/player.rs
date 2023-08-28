use super::*;

impl Model {
    pub(super) fn control_player(&mut self, delta_time: Time) {
        match self.player.state {
            PlayerState::Human => self.human_control(delta_time),
            PlayerState::Barrel { last_gas } => self.barrel_control(last_gas, delta_time),
        };
    }

    fn human_control(&mut self, _delta_time: Time) {
        struct PlayerRef<'a> {
            body: BodyRefMut<'a>,
            controller: &'a mut Controller,
            stats: &'a Stats,
        }

        let player = get!(
            self.actors,
            self.player.actor,
            PlayerRef {
                body: &mut body,
                controller: &mut controller,
                stats,
            }
        );
        let Some(player) = player else { return };

        // Reset rotation
        *player.body.collider.rotation = Angle::ZERO;

        // Update body shape and mass
        *player.body.collider.shape = self.config.player.human_state.body.shape;
        *player.body.mass = self.config.player.human_state.body.mass;

        // Controller
        player.controller.target_velocity = self.player.input.direction * player.stats.move_speed;
        player.controller.acceleration = self.config.player.acceleration;
    }

    fn barrel_control(&mut self, mut last_gas: Position, delta_time: Time) {
        struct PlayerRef<'a> {
            body: BodyRefMut<'a>,
            controller: &'a mut Controller,
        }

        let player = get!(
            self.actors,
            self.player.actor,
            PlayerRef {
                body: &mut body,
                controller: &mut controller,
            }
        );
        let Some(player) = player else { return };

        // Update body shape and mass
        *player.body.collider.shape = self.config.player.barrel_state.body.shape;
        *player.body.mass = self.config.player.barrel_state.body.mass;

        // Controller
        // let input_direction = (self.player.aim_at - *player.position).normalize_or_zero();
        let input_direction = self.player.input.direction;
        let delta_angle = if input_direction == vec2::ZERO {
            Angle::ZERO
        } else {
            let current_angle = player.body.velocity.arg();
            let target_angle = input_direction.arg();
            let delta_angle = current_angle.angle_to(target_angle);
            let steering = self.config.player.barrel_state.steering;
            delta_angle.clamp_abs(Angle::from_radians(steering * delta_time))
        };
        player.controller.target_velocity = player
            .body
            .velocity
            .rotate(delta_angle)
            .clamp_len(..=self.config.player.barrel_state.speed);
        player.controller.acceleration = r32(100.0);

        // Look in the direction of travel
        *player.body.collider.rotation = player.body.velocity.arg();

        // Drip gasoline
        let config = &self.config.player.barrel_state.gasoline;
        if !config.can_control || self.player.input.drip_gas {
            let pos = *player.body.collider.position;
            let last_delta = pos.delta_to(last_gas);
            let last_dir = last_delta.normalize_or_zero();
            let mut last_dist = last_delta.len();
            while last_dist >= config.distance_period {
                let position = last_gas.shifted(-last_dir * config.distance_period);
                last_gas = position;
                last_dist -= config.distance_period;

                if self.player.gasoline.value() < config.cost {
                    break;
                }
                self.player.gasoline.change(-config.cost);
                self.gasoline.insert(Gasoline {
                    collider: Collider::new(position, config.shape),
                    lifetime: Lifetime::new_max(config.lifetime),
                    ignite_timer: config.ignite_timer,
                    fire_radius: config.fire_radius,
                    explosion: config.explosion.clone(),
                    fire: config.fire.clone(),
                });
            }
        } else {
            last_gas = *player.body.collider.position;
        }

        self.player.state = PlayerState::Barrel { last_gas };
    }
}
