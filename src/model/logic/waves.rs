use super::*;

impl Model {
    pub(super) fn update_waves(&mut self, delta_time: Time) {
        // Update difficulty over time
        self.wave_manager.difficulty += delta_time
            * self
                .wave_manager
                .config
                .infinite_wave
                .difficulty_time_scaling;

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
            let pos = rng.gen_circle(vec2::ZERO, self.wave_manager.config.spawn_circle_radius);
            let pos = self.wave_manager.spawn_point.shifted(pos);
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

            if query!(self.actors, (&fraction))
                .any(|(_, (fraction,))| *fraction != Fraction::Player)
            {
                // Some enemies haven't died yet
                return;
            }
        }

        self.wave_manager.difficulty += self
            .wave_manager
            .config
            .infinite_wave
            .difficulty_wave_scaling;

        // Start the next wave
        if let Some(wave) = self.wave_manager.config.waves.pop_front() {
            self.switch_wave(wave);
            return;
        }

        // No more waves found
        if self.wave_manager.infinite_wave_number == self.waves.infinite_waves_until_boss {
            // Spawn boss
            self.boss_wave();
        } else {
            // Infinite wave
            let config = &self.wave_manager.config.infinite_wave;
            let mut wave = WaveConfig {
                spawn_delay: config.spawn_delay,
                wait_for_deaths: true,
                wave_delay: config.wave_delay,
                enemies: VecDeque::new(),
            };

            let mut points = self.wave_manager.difficulty;
            while points > R32::ZERO {
                let Some((enemy, enemy_config)) = config
                .enemies
                .iter()
                .filter(|(_, config)| config.cost <= points)
                .choose(&mut rng)
            else {
                break;
            };
                points -= enemy_config.cost;
                wave.enemies.push_back(enemy.clone());
            }

            self.switch_wave(wave);
        }
        self.wave_manager.infinite_wave_number += 1;
    }

    fn switch_wave(&mut self, wave: WaveConfig) {
        let mut rng = thread_rng();

        // Add missing barrels
        let amount = self
            .blocks
            .kind
            .iter()
            .filter(|(_, kind)| matches!(kind, BlockKind::Barrel))
            .count();
        let amount = 3_usize.saturating_sub(amount); // Expect 3 barrels each wave
        self.add_barrels(amount);

        self.wave_manager.wave_delay = wave.wave_delay;

        let Some(player_pos) = self.get_player_pos() else {
            return;
        };

        let config = &self.wave_manager.config;
        let angle = Angle::from_degrees(r32(rng.gen_range(0.0..=360.0)));
        let distance = rng.gen_range(config.min_spawn_distance..=config.max_spawn_distance);
        self.wave_manager.spawn_point = player_pos.shifted(angle.unit_vec() * distance);

        self.wave_manager.current_wave = wave;
        self.wave_manager.wave_number += 1;
    }

    fn boss_wave(&mut self) {
        // let mut rng = thread_rng();

        // Explode
        self.queued_effects.push_back(QueuedEffect {
            effect: Effect::Explosion {
                position: Position::zero(self.config.world_size),
                config: ExplosionConfig {
                    radius: r32(50.0),
                    knockback: r32(100.0),
                    damage: Hp::ZERO,
                    ignite_gasoline: false,
                    ignite: None,
                },
            },
        });

        // Remove blocks near the spawn
        let to_remove: Vec<Id> = self
            .blocks
            .collider
            .position
            .iter()
            .filter(|(_, pos)| pos.as_dir().len().as_f32() <= 50.0)
            .map(|(id, _)| id)
            .collect();
        for id in to_remove {
            self.blocks.remove(id);
        }

        self.wave_manager.current_wave.wait_for_deaths = true;

        // Feet
        let mut place_foot = |pos: vec2<f32>| {
            let position = Position::from_world(pos.as_r32(), self.config.world_size);
            self.actors.insert(Actor::new_enemy(
                position,
                EnemyConfig {
                    body: BodyConfig {
                        shape: Shape::Rectangle {
                            width: r32(8.0),
                            height: r32(5.0),
                        },
                        mass: r32(1e6),
                    },
                    stats: Stats {
                        contact_damage: r32(50.0),
                        move_speed: r32(0.0),
                        vulnerability: VulnerabilityStats { ..default() },
                    },
                    acceleration: r32(0.0),
                    hp: Hp::new(500.0),
                    ai: ActorAI::BossFoot { position },
                    kind: ActorKind::BossFoot {
                        leg_offset: vec2(0.0, pos.y + 7.0).as_r32(),
                    },
                    gun: None,
                    stops_barrel: true,
                },
            ));
        };

        place_foot(vec2(-28.0, 3.0));
        place_foot(vec2(29.0, 5.0));
        place_foot(vec2(31.0, -7.0));
        place_foot(vec2(-31.0, -9.0));

        // Body
        self.actors.insert(Actor::new_enemy(
            Position::from_world(vec2(0.0, 0.0).as_r32(), self.config.world_size),
            EnemyConfig {
                body: BodyConfig {
                    shape: Shape::Circle { radius: r32(5.0) },
                    mass: r32(1e6),
                },
                stats: Stats {
                    contact_damage: r32(100.0),
                    move_speed: r32(0.0),
                    vulnerability: VulnerabilityStats { ..default() },
                },
                acceleration: r32(0.0),
                hp: Hp::new(1000.0),
                ai: ActorAI::BossBody,
                kind: ActorKind::BossBody,
                gun: None,
                stops_barrel: true,
            },
        ));
    }
}
