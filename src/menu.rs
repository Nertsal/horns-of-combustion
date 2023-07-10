use crate::{
    assets::{
        config::{Config, EnemyConfig, LevelConfig},
        theme::Theme,
        waves::WavesConfig,
        Assets,
    },
    model::{Model, Time},
    prelude::*,
    render::GameRender,
};

const BUTTON_SIZE: vec2<f32> = vec2(5.0, 2.0);
const BUTTON_COLOR: Rgba<f32> = Rgba {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 0.2,
};
const HOVER_COLOR: Rgba<f32> = Rgba {
    r: 0.5,
    g: 0.5,
    b: 0.5,
    a: 0.5,
};
const TEXT_COLOR: Rgba<f32> = Rgba::WHITE;

pub struct StartMenu {
    geng: Geng,
    assets: Rc<Assets>,
    opts: crate::Opts,
    transition: Option<geng::state::Transition>,
    camera: Camera2d,
    framebuffer_size: vec2<usize>,
    render: GameRender,
    model: Model,
    game_texture: ugli::Texture,
    delta_time: Time,
    cursor_pos: vec2<f32>,
    play_button: Aabb2<f32>,
    exit_button: Aabb2<f32>,
    screen_texture: ugli::Texture,
    animation_frame: usize,
    next_frame: f32,
}

impl StartMenu {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        geng: &Geng,
        opts: crate::Opts,
        assets: &Rc<Assets>,
        config: Config,
        level: LevelConfig,
        theme: Theme,
        enemies: HashMap<String, EnemyConfig>,
        waves: WavesConfig,
    ) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            opts,
            transition: None,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 20.0,
            },
            framebuffer_size: vec2(1, 1),
            cursor_pos: vec2::ZERO,
            render: GameRender::new(geng, assets, theme.clone()),
            model: Model::new(
                theme,
                config,
                level,
                enemies,
                WavesConfig {
                    infinite_waves_until_boss: usize::MAX,
                    ..waves
                },
            ),
            play_button: Aabb2::point(vec2(0.0, -1.0)).extend_symmetric(BUTTON_SIZE / 2.0),
            exit_button: Aabb2::point(vec2(0.0, -3.5)).extend_symmetric(BUTTON_SIZE / 2.0),
            game_texture: {
                let mut texture =
                    ugli::Texture::new_with(geng.ugli(), crate::SCREEN_SIZE, |_| Rgba::BLACK);
                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
            screen_texture: {
                let mut texture =
                    ugli::Texture::new_with(geng.ugli(), vec2(1280, 720), |_| Rgba::BLACK);
                texture.set_filter(ugli::Filter::Nearest);
                texture
            },
            delta_time: Time::ONE,
            animation_frame: 0,
            next_frame: assets.sprites.game_logo.first().unwrap().duration,
        }
    }

    fn draw_button(
        &self,
        pos: Aabb2<f32>,
        text: impl AsRef<str>,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let is_hovered = pos.contains(self.cursor_pos);
        let color = if is_hovered {
            HOVER_COLOR
        } else {
            BUTTON_COLOR
        };
        self.geng
            .draw2d()
            .draw2d(framebuffer, &self.camera, &draw2d::Quad::new(pos, color));

        let font = &self.assets.font;
        // self.geng.draw2d().draw2d(
        //     framebuffer,
        //     &self.camera,
        //     &draw2d::Text::unit(font.clone(), text, TEXT_COLOR).fit_into(pos),
        // );
        font.draw(
            framebuffer,
            &self.camera,
            text.as_ref(),
            vec2::splat(geng::TextAlign(0.5)),
            mat3::translate(pos.center())
                * mat3::scale_uniform(1.0)
                * mat3::translate(vec2(0.0, -0.25)),
            TEXT_COLOR,
        )
    }
}

impl geng::State for StartMenu {
    fn transition(&mut self) -> Option<geng::state::Transition> {
        self.transition.take()
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = delta_time as f32;
        self.next_frame -= delta_time;
        let animation = &self.assets.sprites.game_logo;
        if self.animation_frame >= animation.len() {
            self.animation_frame = 0;
        }
        self.next_frame -= delta_time;
        if self.next_frame < 0.0 {
            self.animation_frame += 1;
            self.next_frame = animation
                .get(self.animation_frame)
                .map_or(0.0, |frame| frame.duration);
        }

        self.delta_time = Time::new(delta_time);
        self.model.update(self.delta_time);
        if self.model.time - self.model.time_alive > Time::new(5.0) {
            self.model.revive();
        }
    }

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::CursorMove { position, .. } => {
                self.cursor_pos = self
                    .camera
                    .screen_to_world(self.framebuffer_size.as_f32(), position.as_f32());
            }
            geng::Event::MousePress { .. } => {
                if self.play_button.contains(self.cursor_pos) {
                    self.transition = Some(geng::state::Transition::Push(Box::new(
                        crate::game::run(&self.geng, self.opts.clone()),
                    )));
                } else if self.exit_button.contains(self.cursor_pos) {
                    // TODO: maybe smth different for web
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        self.transition = Some(geng::state::Transition::Pop);
                    }
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.framebuffer_size = framebuffer.size();

        let mut game_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.game_texture),
        );
        ugli::clear(&mut game_framebuffer, Some(Rgba::BLACK), None, None);

        // Game in the background
        self.render
            .draw(&self.model, self.delta_time, &mut game_framebuffer);

        // Draw game to screen
        let framebuffer_size = framebuffer.size().as_f32();
        let texture_size = self.game_texture.size().as_f32();
        let ratio = (framebuffer_size.x / texture_size.x).min(framebuffer_size.y / texture_size.y);
        let texture_size = texture_size * ratio;
        self.geng.draw2d().textured_quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            Aabb2::point(framebuffer_size / 2.0).extend_symmetric(texture_size / 2.0),
            &self.game_texture,
            Rgba::WHITE,
        );

        let mut screen_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.screen_texture),
        );
        ugli::clear(
            &mut screen_framebuffer,
            Some(Rgba::new(80, 30, 20, 180).convert()),
            None,
            None,
        );

        // let aspect = framebuffer_size.aspect();
        // let target = if aspect > 16.0 / 9.0 {
        //     vec2(framebuffer_size.y * 16.0 / 9.0, framebuffer_size.y)
        // } else {
        //     vec2(framebuffer_size.x, framebuffer_size.x * 9.0 / 16.0)
        // };

        let animation = &self.assets.sprites.game_logo;
        if self.animation_frame >= animation.len() {
            self.animation_frame = 0;
        };
        let texture = &animation.get(self.animation_frame).unwrap().texture;

        let framebuffer_size = screen_framebuffer.size().as_f32();
        let size = framebuffer_size.x * 0.8;
        let size = vec2(size, size / texture.size().as_f32().aspect());
        let position = Aabb2::point(framebuffer_size * vec2(0.5, 0.95))
            .extend_symmetric(vec2(size.x / 2.0, 0.0))
            .extend_down(size.y);
        self.geng.draw2d().textured_quad(
            &mut screen_framebuffer,
            &geng::PixelPerfectCamera,
            position,
            texture,
            Rgba::WHITE,
        );

        // Draw texture to actual screen
        let framebuffer_size = framebuffer.size().as_f32();
        let texture_size = self.screen_texture.size().as_f32();
        let ratio = (framebuffer_size.x / texture_size.x).min(framebuffer_size.y / texture_size.y);
        let texture_size = texture_size * ratio;
        self.geng.draw2d().textured_quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            Aabb2::point(framebuffer_size / 2.0).extend_symmetric(texture_size / 2.0),
            &self.screen_texture,
            Rgba::WHITE,
        );

        self.draw_button(self.play_button, "Play", framebuffer);
        self.draw_button(self.exit_button, "Exit", framebuffer);
    }
}

pub fn run(geng: &Geng, opts: crate::Opts) -> impl geng::State {
    let future = {
        let geng = geng.clone();
        async move {
            let manager = geng.asset_manager();
            let assets = Assets::load(manager).await.unwrap();
            let config = Config::load(&opts.config).await.unwrap();
            let level: LevelConfig = crate::util::load_file(&opts.level).await.unwrap();
            let enemies = Config::load_enemies(&opts.enemies).await.unwrap();
            let waves = WavesConfig::load(&opts.waves).await.unwrap();
            let theme = Theme::load(&opts.theme).await.unwrap();
            StartMenu::new(
                &geng,
                opts,
                &Rc::new(assets),
                config,
                level,
                theme,
                enemies,
                waves,
            )
        }
    };
    geng::LoadingScreen::new(geng, geng::EmptyLoadingScreen::new(geng), future)
}
