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

pub struct Game {
    geng: Geng,
    framebuffer_size: vec2<usize>,
    delta_time: Time,
    screen_texture: ugli::Texture,
    controls: Controls,
    can_shoot: bool,
    render: GameRender,
    model: Model,
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
        Self {
            geng: geng.clone(),
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
        let cursor_pos = window.cursor_position().as_f32();
        let aim_position = self
            .model
            .camera
            .screen_to_world(self.framebuffer_size.as_f32(), cursor_pos);
        player.input.aim_at = aim_position.as_r32();

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
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        self.framebuffer_size = framebuffer.size();
        let mut screen_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.screen_texture),
        );

        self.render
            .draw(&self.model, self.delta_time, &mut screen_framebuffer);

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
    }

    fn handle_event(&mut self, event: geng::Event) {
        if is_event_down(&event, &self.controls.fullscreen) {
            let window = self.geng.window();
            window.set_fullscreen(!window.is_fullscreen());
        }

        self.update_player(&event);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.delta_time = delta_time;

        let window = self.geng.window();
        if self.can_shoot && is_key_pressed(window, &self.controls.shoot) {
            let position = window.cursor_position();
            let world_pos = self
                .model
                .camera
                .screen_to_world(self.framebuffer_size.as_f32(), position.as_f32())
                .as_r32();
            self.model.player_action(PlayerAction::Shoot {
                target_pos: Position::from_world(world_pos, self.model.config.world_size),
            });
        }

        self.model.update(delta_time);
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
