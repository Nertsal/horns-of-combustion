use crate::{
    assets::{config::Config, controls::Controls, theme::Theme, Assets},
    model::*,
    render::GameRender,
    util::{is_event_down, is_key_pressed, Vec2RealConversions},
};

use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    framebuffer_size: vec2<usize>,
    controls: Controls,
    render: GameRender,
    model: Model,
}

impl Game {
    pub fn new(
        geng: &Geng,
        assets: &Rc<Assets>,
        config: Config,
        theme: Theme,
        controls: Controls,
    ) -> Self {
        Self {
            geng: geng.clone(),
            framebuffer_size: vec2(1, 1),
            controls,
            render: GameRender::new(geng, assets, theme),
            model: Model::new(config),
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
        player.input_direction = player_direction.normalize_or_zero().as_r32();

        // Aim
        let cursor_pos = window.cursor_position().as_f32();
        let aim_position = self
            .model
            .camera
            .screen_to_world(self.framebuffer_size.as_f32(), cursor_pos);
        player.aim_at = aim_position.as_r32();

        // Transform state
        if is_event_down(event, &self.controls.transform) {
            self.model.player_action(PlayerAction::SwitchState);
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        self.framebuffer_size = framebuffer.size();
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.update_player(&event);
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);

        let window = self.geng.window();
        if is_key_pressed(window, &self.controls.shoot) {
            let position = window.cursor_position();
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
