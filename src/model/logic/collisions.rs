use super::*;

impl Model {
    pub(super) fn collisions(&mut self, delta_time: Time) {
        self.collide_player(delta_time);
        self.collide_actors(delta_time);
        self.collide_projectiles(delta_time);
        self.collide_blocks(delta_time);
        self.projectile_gas(delta_time);
        self.fire_gas(delta_time);
        self.fire(delta_time);
    }

    fn collide_player(&mut self, delta_time: Time) {
        match self.player.state {
            PlayerState::Human => {
                self.collide_player_human(delta_time);
            }
            PlayerState::Barrel { .. } => {
                self.collide_player_barrel(delta_time);
            }
        }

        self.collide_player_pickup(delta_time);
    }

    fn collide_player_pickup(&mut self, _delta_time: Time) {
        struct PlayerRef<'a> {
            collider: ColliderRef<'a>,
            health: &'a mut Health,
        }

        if let Some(player) = get!(
            self.actors,
            self.player.actor,
            PlayerRef {
                collider: &body.collider,
                health: &mut health
            }
        ) {
            let player_collider = player.collider.clone();

            let mut picked_up: Vec<Id> = Vec::new();
            for (pickup_id, (collider,)) in query!(self.pickups, (&body.collider)) {
                if player_collider.check(&collider.clone(), self.config.world_size) {
                    picked_up.push(pickup_id);
                }
            }

            for id in picked_up {
                if player.health.hp < player.health.max_hp {
                    let pickup = self.pickups.remove(id).unwrap();
                    // TODO: as effect
                    match pickup.kind {
                        PickUpKind::Heal { hp } => {
                            player.health.heal(hp);
                            self.queued_effects.push_back(QueuedEffect {
                                effect: Effect::Particles {
                                    position: *player.collider.position,
                                    position_radius: r32(2.0),
                                    velocity: vec2::UNIT_Y,
                                    size: r32(0.2),
                                    lifetime: r32(1.0),
                                    intensity: hp,
                                    kind: ParticleKind::Heal,
                                },
                            });
                        }
                    }
                }
            }
        }
    }

    fn collide_player_human(&mut self, _delta_time: Time) {
        struct ActorRef<'a> {
            collider: ColliderRef<'a>,
            velocity: &'a vec2<Coord>,
            stats: &'a Stats,
            health: &'a Health,
        }

        let player = get!(
            self.actors,
            self.player.actor,
            ActorRef {
                collider: &body.collider,
                velocity: &body.velocity,
                stats,
                health,
            }
        );
        let Some(player) = player else { return };
        let player_collider = &player.collider.clone();

        #[derive(Clone)]
        struct Correction {
            position: Position,
            velocity: vec2<Coord>,
            health: Health,
        }
        let mut corrections: HashMap<Id, Correction> = HashMap::new();

        for (actor_id, actor) in query!(
            self.actors,
            ActorRef {
                collider: &body.collider,
                velocity: &body.velocity,
                stats,
                health,
            }
        ) {
            if actor_id == self.player.actor {
                continue;
            }
            if let Some(collision) =
                player_collider.collide(&actor.collider.clone(), self.config.world_size)
            {
                let mut player_cor =
                    corrections
                        .get(&self.player.actor)
                        .cloned()
                        .unwrap_or(Correction {
                            position: *player.collider.position,
                            velocity: *player.velocity,
                            health: player.health.clone(),
                        });
                let mut actor_cor = corrections.get(&actor_id).cloned().unwrap_or(Correction {
                    position: *actor.collider.position,
                    velocity: *actor.velocity,
                    health: actor.health.clone(),
                });

                let relative_vel = player_cor.velocity - *actor.velocity;
                let dot = vec2::dot(relative_vel, collision.normal);
                if dot <= Coord::ZERO {
                    continue;
                }

                let coef_player = r32(3.0);
                let coef_actor = r32(1.0);

                // Move out of collision
                player_cor.position.shift(
                    -collision.normal * collision.penetration * coef_player
                        / (coef_player + coef_actor),
                    self.config.world_size,
                );
                actor_cor.position.shift(
                    collision.normal * collision.penetration * coef_actor
                        / (coef_player + coef_actor),
                    self.config.world_size,
                );

                // Apply impulses
                let hit_strength = dot.min(r32(10.0));
                player_cor.velocity -= collision.normal * hit_strength * coef_player;
                actor_cor.velocity += collision.normal * hit_strength * coef_actor;

                if dot > r32(10.0) {
                    // Contact damage
                    let damage_actor = self.config.player.stats.contact_damage
                        * actor.stats.vulnerability.physical;
                    let damage_player = actor.stats.contact_damage
                        * self.config.player.stats.vulnerability.physical;
                    player_cor.health.damage(damage_player);
                    actor_cor.health.damage(damage_actor);

                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::particles_damage(player_cor.position, damage_player),
                    });
                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::particles_damage(actor_cor.position, damage_actor),
                    });
                }

                corrections.insert(self.player.actor, player_cor);
                corrections.insert(actor_id, actor_cor);
            }
        }

        struct UpdateRef<'a> {
            position: &'a mut Position,
            velocity: &'a mut vec2<Coord>,
            health: &'a mut Health,
        }

        for (id, correction) in corrections {
            let actor = get!(
                self.actors,
                id,
                UpdateRef {
                    position: &mut body.collider.position,
                    velocity: &mut body.velocity,
                    health: &mut health
                }
            )
            .expect("invalid correction");
            *actor.position = correction.position;
            *actor.velocity = correction.velocity;
            *actor.health = correction.health;
        }
    }

    fn collide_player_barrel(&mut self, _delta_time: Time) {
        struct ActorRef<'a> {
            collider: ColliderRef<'a>,
            velocity: &'a vec2<Coord>,
            stats: &'a Stats,
            stops_barrel: &'a bool,
            health: &'a Health,
        }

        let player = get!(
            self.actors,
            self.player.actor,
            ActorRef {
                collider: &body.collider,
                velocity: &body.velocity,
                stats,
                stops_barrel,
                health,
            }
        );
        let Some(player) = player else { return };
        let player_collider = &player.collider.clone();

        #[derive(Clone)]
        struct Correction {
            position: Position,
            velocity: vec2<Coord>,
            stun: Option<Time>,
            health: Health,
        }
        let mut corrections: HashMap<Id, Correction> = HashMap::new();

        for (actor_id, actor) in query!(
            self.actors,
            ActorRef {
                collider: &body.collider,
                velocity: &body.velocity,
                stats,
                stops_barrel,
                health,
            }
        ) {
            if actor_id == self.player.actor {
                continue;
            }
            if let Some(collision) =
                player_collider.collide(&actor.collider.clone(), self.config.world_size)
            {
                let mut player_cor =
                    corrections
                        .get(&self.player.actor)
                        .cloned()
                        .unwrap_or(Correction {
                            position: *player.collider.position,
                            velocity: *player.velocity,
                            stun: None,
                            health: player.health.clone(),
                        });
                let mut actor_cor = corrections.get(&actor_id).cloned().unwrap_or(Correction {
                    position: *actor.collider.position,
                    velocity: *actor.velocity,
                    stun: None,
                    health: actor.health.clone(),
                });

                let relative_vel = player_cor.velocity - *actor.velocity;
                let dot = vec2::dot(relative_vel, collision.normal);
                if dot <= Coord::ZERO {
                    continue;
                }

                // Apply impulses
                let hit_barrel = if *actor.stops_barrel {
                    dot
                } else {
                    dot.min(r32(2.0))
                };
                player_cor.velocity -= collision.normal * hit_barrel;

                // TODO: fix double damage (with on collision enter or smth)
                // Runover damage
                let damage = self.config.player.barrel_state.runover_damage
                    + self.config.player.barrel_state.runover_damage_scale * relative_vel.len();
                let actor_damage = damage * actor.stats.vulnerability.physical;
                actor_cor.health.damage(actor_damage);

                self.queued_effects.push_back(QueuedEffect {
                    effect: Effect::particles_damage(actor_cor.position, actor_damage),
                });

                // TODO: configurable + better formula
                actor_cor.stun = if *actor.stops_barrel {
                    None
                } else {
                    Some(r32(3.0))
                };

                corrections.insert(self.player.actor, player_cor);
                corrections.insert(actor_id, actor_cor);
            }
        }

        struct UpdateRef<'a> {
            position: &'a mut Position,
            velocity: &'a mut vec2<Coord>,
            stunned: &'a mut Option<Time>,
            health: &'a mut Health,
        }

        for (id, correction) in corrections {
            let actor = get!(
                self.actors,
                id,
                UpdateRef {
                    position: &mut body.collider.position,
                    velocity: &mut body.velocity,
                    stunned: &mut stunned,
                    health: &mut health,
                }
            )
            .expect("invalid correction");
            *actor.position = correction.position;
            *actor.velocity = correction.velocity;
            *actor.stunned = std::cmp::max(*actor.stunned, correction.stun);
            *actor.health = correction.health;
        }
    }

    fn collide_actors(&mut self, _delta_time: Time) {
        struct ActorRef<'a> {
            collider: ColliderRef<'a>,
            velocity: &'a vec2<Coord>,
        }

        #[derive(Clone)]
        struct Correction {
            position: Position,
            velocity: vec2<Coord>,
        }

        let mut corrections: HashMap<Id, Correction> = HashMap::new();

        for (actor_id, actor) in query!(
            self.actors,
            ActorRef {
                collider: &body.collider,
                velocity: &body.velocity,
            }
        ) {
            if actor_id == self.player.actor {
                continue;
            }

            let mut actor_collider = actor.collider.clone();
            let mut actor_cor = corrections.get(&actor_id).cloned().unwrap_or(Correction {
                position: *actor.collider.position,
                velocity: *actor.velocity,
            });

            for (other_id, other) in query!(
                self.actors,
                ActorRef {
                    collider: &body.collider,
                    velocity: &body.velocity,
                }
            ) {
                if other_id == self.player.actor || other_id <= actor_id {
                    continue;
                }

                let mut other_collider = other.collider.clone();
                let mut other_cor = corrections.get(&other_id).cloned().unwrap_or(Correction {
                    position: *other.collider.position,
                    velocity: *other.velocity,
                });

                actor_collider.position = actor_cor.position;
                other_collider.position = other_cor.position;

                if let Some(collision) =
                    actor_collider.collide(&other.collider.clone(), self.config.world_size)
                {
                    actor_cor.position.shift(
                        -collision.normal * collision.penetration / r32(2.0),
                        self.config.world_size,
                    );
                    other_cor.position.shift(
                        collision.normal * collision.penetration / r32(2.0),
                        self.config.world_size,
                    );

                    corrections.insert(other_id, other_cor);
                }
            }

            corrections.insert(actor_id, actor_cor);
        }

        for (actor, correction) in corrections {
            let (position, velocity) = get!(
                self.actors,
                actor,
                (&mut body.collider.position, &mut body.velocity)
            )
            .unwrap();

            *position = correction.position;
            *velocity = correction.velocity;
        }
    }

    fn collide_projectiles(&mut self, _delta_time: Time) {
        struct ProjRef<'a> {
            fraction: &'a Fraction,
            collider: ColliderRef<'a>,
            velocity: &'a mut vec2<Coord>,
            damage: &'a Hp,
            knockback: &'a Coord,
        }

        struct ActorRef<'a> {
            fraction: &'a Fraction,
            collider: ColliderRef<'a>,
            velocity: &'a mut vec2<Coord>,
            health: &'a mut Health,
            stats: &'a Stats,
        }

        let mut proj_hits: Vec<Id> = Vec::new();
        for proj_id in self.projectiles.ids() {
            let proj = get!(
                self.projectiles,
                proj_id,
                ProjRef {
                    fraction,
                    collider: &body.collider,
                    velocity: &mut body.velocity,
                    damage,
                    knockback,
                }
            );
            let Some(proj) = proj else { continue };

            for actor_id in self.actors.ids() {
                let actor = get!(
                    self.actors,
                    actor_id,
                    ActorRef {
                        fraction,
                        collider: &body.collider,
                        velocity: &mut body.velocity,
                        health: &mut health,
                        stats,
                    }
                );
                let Some(actor) = actor else { continue };

                if proj.fraction == actor.fraction {
                    continue;
                }
                if proj
                    .collider
                    .clone()
                    .check(&actor.collider.clone(), self.config.world_size)
                {
                    proj_hits.push(proj_id);
                    actor
                        .health
                        .damage(*proj.damage * actor.stats.vulnerability.projectile);

                    // If player is hit, switch back to human state
                    if *actor.fraction == Fraction::Player {
                        self.player.state = PlayerState::Human;
                    }

                    let relative_vel = *proj.velocity - *actor.velocity;
                    // let dot = vec2::dot(relative_vel, collision.normal);

                    // Knockback
                    *actor.velocity += relative_vel * r32(0.1) * *proj.knockback;

                    self.queued_effects.push_back(QueuedEffect {
                        effect: Effect::particles_damage(*actor.collider.position, *proj.damage),
                    });

                    break;
                }
            }
        }

        for id in proj_hits {
            self.projectiles.remove(id);
        }
    }

    fn collide_blocks(&mut self, _delta_time: Time) {
        struct BlockRef<'a> {
            collider: ColliderRef<'a>,
            health: &'a mut Option<Health>,
            vulnerability: &'a VulnerabilityStats,
        }

        // Actors

        struct ActorRef<'a> {
            collider: ColliderRefMut<'a>,
            velocity: &'a mut vec2<Coord>,
        }

        for actor_id in self.actors.ids() {
            let actor = get!(
                self.actors,
                actor_id,
                ActorRef {
                    collider: &mut body.collider,
                    velocity: &mut body.velocity,
                }
            );
            let Some(actor) = actor else { continue };

            for block_id in self.blocks.ids() {
                let block = get!(
                    self.blocks,
                    block_id,
                    BlockRef {
                        collider,
                        health: &mut health,
                        vulnerability,
                    }
                );
                let Some(block) = block else { continue };

                if let Some(collision) = actor
                    .collider
                    .clone()
                    .collide(&block.collider.clone(), self.config.world_size)
                {
                    actor.collider.position.shift(
                        -collision.normal * collision.penetration,
                        self.config.world_size,
                    );

                    let dot = vec2::dot(collision.normal, *actor.velocity);
                    let bounciness = r32(0.5);
                    *actor.velocity -= collision.normal * dot * (Coord::ONE + bounciness);
                }
            }
        }

        // Projectiles

        struct ProjRef<'a> {
            collider: ColliderRefMut<'a>,
            damage: &'a Hp,
        }

        let mut hit_projs: Vec<Id> = Vec::new();
        for proj_id in self.projectiles.ids() {
            let proj = get!(
                self.projectiles,
                proj_id,
                ProjRef {
                    collider: &mut body.collider,
                    damage,
                }
            );
            let Some(proj) = proj else { continue };

            for block_id in self.blocks.ids() {
                let block = get!(
                    self.blocks,
                    block_id,
                    BlockRef {
                        collider,
                        health: &mut health,
                        vulnerability,
                    }
                );
                let Some(block) = block else { continue };

                if proj
                    .collider
                    .clone()
                    .check(&block.collider.clone(), self.config.world_size)
                {
                    hit_projs.push(proj_id);
                    if let Some(health) = block.health {
                        health.damage(*proj.damage * block.vulnerability.physical);
                    }
                    break;
                }
            }
        }

        for id in hit_projs {
            let proj = self.projectiles.remove(id).unwrap();
            self.queued_effects.push_back(QueuedEffect {
                effect: {
                    Effect::Particles {
                        position: proj.body.collider.position,
                        position_radius: r32(1.0),
                        velocity: vec2::UNIT_Y,
                        size: r32(0.2),
                        lifetime: r32(1.0),
                        intensity: r32(20.0),
                        kind: ParticleKind::Projectile,
                    }
                },
            });
        }
    }

    /// Projectiles ignite gas when passing over it.
    fn projectile_gas(&mut self, _delta_time: Time) {
        struct ProjRef<'a> {
            collider: ColliderRef<'a>,
        }

        struct GasRef<'a> {
            collider: ColliderRef<'a>,
        }

        let mut gas_ignited: Vec<Id> = Vec::new();

        for (gas_id, gas) in query!(self.gasoline, GasRef { collider }) {
            for (_proj_id, proj) in query!(
                self.projectiles,
                ProjRef {
                    collider: &body.collider
                }
            ) {
                if proj
                    .collider
                    .clone()
                    .check(&gas.collider.clone(), self.config.world_size)
                {
                    gas_ignited.push(gas_id);
                    break;
                }
            }
        }

        for id in gas_ignited {
            self.ignite_gasoline(id);
        }
    }

    fn fire_gas(&mut self, delta_time: Time) {
        struct FireRef<'a> {
            collider: ColliderRef<'a>,
        }

        struct GasRef<'a> {
            collider: ColliderRef<'a>,
            ignite_timer: &'a mut Time,
        }

        let mut to_ignite: Vec<Id> = Vec::new();
        for gas_id in self.gasoline.ids() {
            let gas = get!(
                self.gasoline,
                gas_id,
                GasRef {
                    collider,
                    ignite_timer: &mut ignite_timer
                }
            );
            let Some(gas) = gas else { continue };

            for (_, fire) in query!(self.fire, FireRef { collider }) {
                if fire
                    .collider
                    .clone()
                    .check(&gas.collider.clone(), self.config.world_size)
                {
                    *gas.ignite_timer -= delta_time;
                    if *gas.ignite_timer <= Time::ZERO {
                        to_ignite.push(gas_id);
                        break;
                    }
                }
            }
        }

        for gas_id in to_ignite {
            self.ignite_gasoline(gas_id);
        }
    }

    fn fire(&mut self, _delta_time: Time) {
        struct FireRef<'a> {
            collider: ColliderRef<'a>,
            config: &'a FireConfig,
        }

        struct ActorRef<'a> {
            collider: ColliderRef<'a>,
            velocity: &'a mut vec2<Coord>,
            stats: &'a Stats,
            on_fire: &'a mut Option<OnFire>,
        }

        for actor_id in self.actors.ids() {
            let actor = get!(
                self.actors,
                actor_id,
                ActorRef {
                    collider: &body.collider,
                    velocity: &mut body.velocity,
                    stats,
                    on_fire: &mut on_fire,
                }
            );
            let Some(actor) = actor else { continue };

            for (_, fire) in query!(self.fire, FireRef { collider, config }) {
                if actor
                    .collider
                    .clone()
                    .check(&fire.collider.clone(), self.config.world_size)
                {
                    if actor.stats.vulnerability.fire > R32::ZERO {
                        *actor.on_fire = Some(update_on_fire(
                            actor.on_fire.clone(),
                            OnFire {
                                duration: fire.config.duration,
                                damage_per_second: fire.config.damage_per_second,
                            },
                        ));
                    }
                    if actor_id == self.player.actor {
                        if let PlayerState::Barrel { .. } = self.player.state {
                            // Explode the barrel
                            // let dir = fire
                            //     .collider
                            //     .position
                            //     .direction(*actor.collider.position, self.config.world_size)
                            //     .normalize_or_zero();
                            let dir = actor.velocity.normalize_or_zero();
                            *actor.velocity +=
                                dir * self.config.player.barrel_state.self_explosion_strength;
                            self.player.state = PlayerState::Human;
                        }
                    }
                }
            }
        }
    }
}
