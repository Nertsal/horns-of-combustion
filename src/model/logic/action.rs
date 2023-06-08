use super::*;

impl Model {
    pub fn player_action(&mut self, action: PlayerAction) {
        match action {
            PlayerAction::Shoot { target_pos } => {
                if let Some(gun) = &mut self.player.actor.gun {
                    if gun.shot_delay <= Time::ZERO {
                        let pos = self.player.actor.body.collider.position;
                        gun.shot_delay = gun.config.shot_delay;
                        self.projectiles.insert(Projectile::new(
                            pos,
                            target_pos,
                            gun.config.projectile,
                        ));
                    }
                }
            }
        }
    }
}
