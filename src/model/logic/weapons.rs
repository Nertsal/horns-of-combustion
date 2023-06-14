use super::*;

impl Model {
    pub fn shoot(
        &mut self,
        position: Position,
        aimed_towards: Position,
        fraction: Fraction,
        config: ShotConfig,
    ) {
        let aim_angle =
            Angle::from_radians((position.direction(aimed_towards, self.config.world_size)).arg());

        let mut shoot_at = |angle: Angle<R32>| {
            self.projectiles.insert(Projectile::new(
                position,
                angle,
                fraction,
                config.projectile.clone(),
            ));
        };

        match config.pattern {
            ShotPattern::Single => shoot_at(aim_angle),
            ShotPattern::Multiple {
                spread_degrees,
                bullets,
            } => {
                for i in 0..bullets {
                    let i = i as f32 / (bullets - 1) as f32 - 0.5;
                    shoot_at(aim_angle + Angle::from_degrees(spread_degrees * r32(i)));
                }
            }
        }

        // Play sound
        self.game_events.push(GameEvent::PlaySound {
            sound: Sound::Shoot,
            volume: self.get_volume_from(position).as_f32(),
        })
    }

    pub fn update_weapons(&mut self, delta_time: Time) {
        self.update_actors(delta_time);
    }

    fn update_actors(&mut self, delta_time: Time) {
        #[allow(dead_code)]
        #[derive(StructQuery)]
        struct ActorRef<'a> {
            #[query(optic = "._Some")]
            gun: &'a mut Gun,
        }

        let mut query = query_actor_ref!(self.actors);
        let mut iter = query.iter_mut();
        while let Some((_id, actor)) = iter.next() {
            update_weapon(&mut actor.gun.shot_delay, delta_time);
        }
    }
}

fn update_weapon(shot_delay: &mut Time, delta_time: Time) {
    *shot_delay = (*shot_delay - delta_time).max(Time::ZERO);
}
