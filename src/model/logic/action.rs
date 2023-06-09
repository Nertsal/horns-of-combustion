use super::*;

impl Model {
    pub fn player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Shoot { target_pos } => {
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
                    self.projectiles.insert(Projectile::new(
                        pos,
                        target_pos,
                        Fraction::Player,
                        player.gun.config.projectile,
                    ));
                }
            }
            PlayerAction::SwitchState => {
                self.player.state = match self.player.state {
                    PlayerState::Human => PlayerState::Barrel {
                        next_gas: self.config.player.barrel_state.gasoline.distance_period,
                    },
                    PlayerState::Barrel { .. } => PlayerState::Human,
                };
            }
        }
    }
}
