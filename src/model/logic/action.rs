use super::*;

impl Model {
    pub fn player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Shoot { target_pos } => {
                if let PlayerState::Human = self.player.state {
                    #[allow(dead_code)]
                    #[derive(StructQuery)]
                    struct PlayerRef<'a> {
                        #[query(storage = ".body.collider")]
                        position: &'a vec2<Coord>,
                        #[query(optic = "._Some")]
                        gun: &'a mut Gun,
                    }

                    let mut query = query_player_ref!(self.actors);
                    let player = query
                        .get_mut(self.player.actor)
                        .expect("Player actor not found");

                    if player.gun.shot_delay <= Time::ZERO {
                        let pos = *player.position;
                        player.gun.shot_delay = player.gun.config.shot_delay;
                        let config = player.gun.config.shot.clone();
                        self.shoot(pos, target_pos, Fraction::Player, config);
                    }
                }
            }
            PlayerAction::SwitchState => {
                self.player.state = match self.player.state {
                    PlayerState::Human => {
                        #[allow(dead_code)]
                        #[derive(StructQuery)]
                        struct PlayerRef<'a> {
                            #[query(storage = ".body.collider")]
                            position: &'a vec2<Coord>,
                            #[query(storage = ".body")]
                            velocity: &'a mut vec2<Coord>,
                        }

                        let mut query = query_player_ref!(self.actors);
                        let player = query
                            .get_mut(self.player.actor)
                            .expect("Player actor not found");

                        // let input_direction =
                        //     (self.player.aim_at - *player.position).normalize_or_zero();
                        let input_direction = self.player.input_direction;
                        let dash_speed = (vec2::dot(*player.velocity, input_direction)
                            .max(Coord::ZERO)
                            + self.config.player.dash_burst)
                            .min(self.config.player.barrel_state.speed);
                        *player.velocity = input_direction * dash_speed;

                        PlayerState::Barrel {
                            next_gas: self.config.player.barrel_state.gasoline.distance_period,
                        }
                    }
                    PlayerState::Barrel { .. } => PlayerState::Human,
                };
            }
        }
    }
}
