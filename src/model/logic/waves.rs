use super::*;

impl Model {
    pub(super) fn update_waves(&mut self, delta_time: Time) {
        // Starting delay
        if self.wave_manager.wave_delay > Time::ZERO {
            self.wave_manager.wave_delay -= delta_time;
            return;
        }

        // Delay between each enemy
        if self.wave_manager.spawn_delay > Time::ZERO {
            self.wave_manager.spawn_delay -= delta_time;
            return;
        }

        let mut rng = thread_rng();

        if let Some(enemy_name) = self.wave_manager.current_wave.enemies.pop_front() {
            // Spawn the next enemy
            let enemy_config = self
                .enemies_list
                .get(&enemy_name)
                .unwrap_or_else(|| panic!("Enemy {:?} not found", enemy_name))
                .clone();
            let pos = rng.gen_circle(
                self.wave_manager.spawn_point,
                self.wave_manager.config.spawn_circle_radius,
            );
            let _enemy = self.actors.insert(Actor::new_enemy(pos, enemy_config));
            // self.wave_manager.current_enemies.push(enemy);
            self.wave_manager.spawn_delay = self.wave_manager.current_wave.spawn_delay;
            return;
        }

        // Check for the end of the wave
        if self.wave_manager.current_wave.wait_for_deaths {
            // self.wave_manager
            //     .current_enemies
            //     .retain(|&id| self.actors.get(id).is_some());
            // if !self.wave_manager.current_enemies.is_empty() {
            //     // Some enemies haven't died yet
            //     return;
            // }

            #[allow(dead_code)]
            #[derive(StructQuery)]
            struct ActorRef<'a> {
                fraction: &'a Fraction,
            }

            let query = query_actor_ref!(self.actors);
            if query
                .iter()
                .any(|(_, actor)| *actor.fraction != Fraction::Player)
            {
                // Some enemies haven't died yet
                return;
            }
        }

        // Start the next wave
        if let Some(wave) = self.wave_manager.config.waves.pop_front() {
            self.wave_manager.wave_delay = wave.wave_delay;

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
            let player_pos = *player.position;

            let config = &self.wave_manager.config;
            let angle = Angle::from_degrees(r32(rng.gen_range(0.0..=360.0)));
            let distance = rng.gen_range(config.min_spawn_distance..=config.max_spawn_distance);
            self.wave_manager.spawn_point = player_pos + angle.unit_vec() * distance;

            self.wave_manager.current_wave = wave;
        }

        // TODO: infinite waves
    }
}
