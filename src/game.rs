use crate::{
    assets::{
        config::{Config, EnemyConfig, LevelConfig},
        controls::Controls,
        theme::Theme,
        waves::WavesConfig,
        Assets,
    },
    model::*,
    render::GameRender,
    util::{is_event_down, is_event_up, is_key_pressed, Vec2RealConversions},
};

use geng::prelude::*;

#[derive(Debug)]
pub enum GameEvent {
    PlaySound { sound: Sound, volume: R32 },
}

#[derive(Debug, Clone, Copy)]
pub enum Sound {
    Shoot,
    Explosion,
}

pub struct Game {
    geng: Geng,
    assets: Rc<Assets>,
    framebuffer_size: vec2<usize>,
    delta_time: Time,
    screen_texture: ugli::Texture,
    controls: Controls,
    can_shoot: bool,
    render: GameRender,
    model: Model,
    master_volume: f32,
    // music_volume: f32,
    explosion_timeout: f32,
}

impl Game {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        config: Config,
        level: LevelConfig,
        theme: Theme,
        controls: Controls,
        enemies: HashMap<String, EnemyConfig>,
        waves: WavesConfig,
    ) -> Self {
        geng.window().set_cursor_type(geng::CursorType::None);
        let mut effect = assets.sounds.music.play();
        effect.set_volume(0.07);

        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            framebuffer_size: vec2(1, 1),
            delta_time: Time::new(1.0),
            screen_texture: {
                let mut texture =
                    ugli::Texture::new_with(geng.ugli(), crate::SCREEN_SIZE, |_| Rgba::BLACK);
                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
            controls,
            can_shoot: true,
            model: Model::new(theme.clone(), config, level, enemies, waves),
            render: GameRender::new(geng, assets, theme),
            master_volume: 0.5,
            // music_volume: 1.0,
            explosion_timeout: 0.0,
        }
    }

    fn update_player(&mut self, event: &geng::Event) {
        let player = &mut self.model.player;
        let window = self.geng.window();

        // Change player velocity based on input.
        let mut player_direction: vec2<f32> = vec2::ZERO;
        if is_key_pressed(window, &self.controls.up) {
            player_direction.y += 1.0;
        }
        if is_key_pressed(window, &self.controls.down) {
            player_direction.y -= 1.0;
        }
        if is_key_pressed(window, &self.controls.right) {
            player_direction.x += 1.0;
        }
        if is_key_pressed(window, &self.controls.left) {
            player_direction.x -= 1.0;
        }

        // Assign normalized
        player.input.direction = player_direction.normalize_or_zero().as_r32();

        // Aim
        player.input.aim_at = self.model.camera.cursor_pos_world();

        // Drip gasoline
        player.input.drip_gas = is_key_pressed(window, &self.controls.gas);

        // Transform state
        if is_event_down(event, &self.controls.transform) {
            self.model.player_action(PlayerAction::SwitchState);
        }

        // Barrel dash
        if let PlayerState::Barrel { .. } = self.model.player.state {
            if is_event_down(event, &self.controls.barrel_dash) {
                self.model.player_action(PlayerAction::BarrelDash);
                self.can_shoot = false;
            }
        } else if is_event_up(event, &self.controls.barrel_dash) {
            self.can_shoot = true;
        };
    }

    /// Restart the game
    fn reset(&mut self) {
        self.model.reset();
    }

    fn handle_game_event(&mut self, event: GameEvent) {
        match event {
            GameEvent::PlaySound { sound, volume } => {
                let volume = volume.as_f32() * self.master_volume * 0.3;
                let (sound, volume_mult) = match sound {
                    Sound::Shoot => (&self.assets.sounds.shoot, 1.0),
                    Sound::Explosion => {
                        if self.explosion_timeout > 0.0 {
                            return;
                        }
                        self.explosion_timeout = 0.2;
                        (&self.assets.sounds.explosion, 0.7)
                    }
                };
                let mut sound = sound.play();
                sound.set_volume((volume * volume_mult).into());
            }
        }
    }
}

impl geng::State for Game {
    fn update(&mut self, delta_time: f64) {
        // Update cursor position within a camera
        self.model.camera.cursor_pos = self.geng.window().cursor_position();
        self.model.camera.framebuffer_size = self.framebuffer_size;

        let delta_time = delta_time as f32;
        self.explosion_timeout -= delta_time;

        let delta_time = Time::new(delta_time);
        self.delta_time = delta_time;

        let window = self.geng.window();
        if self.can_shoot && is_key_pressed(window, &self.controls.shoot) {
            let target_pos = self.model.camera.cursor_pos_world();
            self.model.player_action(PlayerAction::Shoot { target_pos });
        }

        for event in self.model.update(delta_time) {
            self.handle_game_event(event);
        }
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        self.framebuffer_size = framebuffer.size();
        let mut screen_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.screen_texture),
        );

        // Draw pixelated world
        self.render
            .draw(&self.model, self.delta_time, &mut screen_framebuffer);

        // Draw cursor
        let texture = match self.model.player.state {
            PlayerState::Human => &self.assets.sprites.crosshair,
            PlayerState::Barrel { .. } => &self.assets.sprites.crosshair_barrel,
        };
        let pos = self.geng.window().cursor_position().as_f32();
        let pos = self
            .model
            .camera
            .screen_to_world(self.framebuffer_size.as_f32(), pos);
        let pos = crate::render::pixel_perfect_aabb(pos, texture.size(), &self.model.camera);
        self.geng.draw2d().textured_quad(
            &mut screen_framebuffer,
            &self.model.camera,
            pos,
            texture,
            Rgba::WHITE,
        );

        // Draw texture to actual screen
        let framebuffer_size = framebuffer.size().as_f32();
        let texture_size = crate::SCREEN_SIZE.as_f32();
        let ratio = (framebuffer_size.x / texture_size.x).min(framebuffer_size.y / texture_size.y);
        let texture_size = texture_size * ratio;
        self.geng.draw2d().textured_quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            Aabb2::point(framebuffer_size / 2.0).extend_symmetric(texture_size / 2.0),
            &self.screen_texture,
            Rgba::WHITE,
        );

        // Draw ui (not pixelated)
        self.render.draw_ui(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        if is_event_down(&event, &self.controls.fullscreen) {
            let window = self.geng.window();
            window.set_fullscreen(!window.is_fullscreen());
        }

        if is_event_down(&event, &self.controls.reset) {
            let player_alive = self.model.time_alive == self.model.time;
            if !player_alive || self.geng.window().is_key_pressed(geng::Key::LCtrl) {
                self.reset()
            }
        }

        self.update_player(&event);
    }
}

pub fn run(geng: &Geng, opts: crate::Opts) -> impl geng::State {
    let future = {
        let geng = geng.clone();
        async move {
            let manager = geng.asset_manager();
            let assets = Assets::load(manager).await.unwrap();
            let config = Config::load(&opts.config).await.unwrap();
            let level: LevelConfig = file::load_detect(&opts.level).await.unwrap();
            let enemies = Config::load_enemies(&opts.enemies).await.unwrap();
            let waves = WavesConfig::load(&opts.waves).await.unwrap();
            let theme = Theme::load(&opts.theme).await.unwrap();
            let controls = Controls::load(&opts.controls).await.unwrap();
            Game::new(
                &geng,
                &Rc::new(assets),
                config,
                level,
                theme,
                controls,
                enemies,
                waves,
            )
        }
    };
    geng::LoadingScreen::new(geng, geng::EmptyLoadingScreen::new(geng), future)
}
