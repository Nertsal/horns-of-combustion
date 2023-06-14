use crate::{assets::Assets, util::Vec2RealConversions};

use geng::prelude::*;

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
    cursor_pos: vec2<f32>,
    play_button: Aabb2<f32>,
    exit_button: Aabb2<f32>,
}

impl StartMenu {
    pub fn new(geng: &Geng, opts: crate::Opts, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            opts,
            transition: None,
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: 0.0,
                fov: 20.0,
            },
            framebuffer_size: vec2(1, 1),
            cursor_pos: vec2::ZERO,
            play_button: Aabb2::point(vec2(0.0, 0.0)).extend_symmetric(BUTTON_SIZE / 2.0),
            exit_button: Aabb2::point(vec2(0.0, -5.0)).extend_symmetric(BUTTON_SIZE / 2.0),
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

    fn handle_event(&mut self, event: geng::Event) {
        match event {
            geng::Event::MouseMove { position, .. } => {
                self.cursor_pos = self
                    .camera
                    .screen_to_world(self.framebuffer_size.as_f32(), position.as_f32());
            }
            geng::Event::MouseDown { position, .. } => {
                self.cursor_pos = self
                    .camera
                    .screen_to_world(self.framebuffer_size.as_f32(), position.as_f32());
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
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

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
            StartMenu::new(&geng, opts, &Rc::new(assets))
        }
    };
    geng::LoadingScreen::new(geng, geng::EmptyLoadingScreen::new(geng), future)
}
