use crate::util::Vec2RealConversions;
use crate::{
    assets::{config::Config, Assets},
    model::*,
    render::GameRender,
};

use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    framebuffer_size: vec2<usize>,
    render: GameRender,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, config: Config) -> Self {
        Self {
            geng: geng.clone(),
            framebuffer_size: vec2(1, 1),
            render: GameRender::new(geng, assets),
            model: Model::new(config),
        }
    }

    fn update_player(&mut self) {
        let player = &mut self.model.player;

        // Change player velocity based on input.
        let mut player_direction: vec2<f32> = vec2::ZERO;
        if self.geng.window().is_key_pressed(geng::Key::W)
            || self.geng.window().is_key_pressed(geng::Key::Up)
        {
            player_direction.y += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::S)
            || self.geng.window().is_key_pressed(geng::Key::Down)
        {
            player_direction.y -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::D)
            || self.geng.window().is_key_pressed(geng::Key::Right)
        {
            player_direction.x += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::A)
            || self.geng.window().is_key_pressed(geng::Key::Left)
        {
            player_direction.x -= 1.0;
        }

        // Normalize player direction.
        player_direction = player_direction.normalize_or_zero();

        player.input_direction = player_direction.as_r32();
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.update_player();

        if let geng::Event::KeyDown {
            key: geng::Key::Space,
        } = event
        {
            self.model.player_action(PlayerAction::SwitchState);
        }
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);

        if self
            .geng
            .window()
            .is_button_pressed(geng::MouseButton::Left)
        {
            let position = self.geng.window().cursor_position();
            let world_pos = self
                .model
                .camera
                .screen_to_world(self.framebuffer_size.as_f32(), position.as_f32())
                .as_r32();
            self.model.player_action(PlayerAction::Shoot {
                target_pos: world_pos,
            });
        }

        self.model.update(delta_time);
    }
}
