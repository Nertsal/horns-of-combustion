use crate::{assets::Assets, model::*, render::GameRender};
use crate::util::{Vec2RealConversions};

use geng::prelude::*;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    render: GameRender,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, _event: geng::Event) {
        let player = &mut self.model.player;

        // Change player velocity based on input.
        let mut player_direction: vec2<f32> = vec2::ZERO;
        if self.geng.window().is_key_pressed(geng::Key::W) {
            player_direction.y += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::S) {
            player_direction.y -= 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::D) {
            player_direction.x += 1.0;
        }
        if self.geng.window().is_key_pressed(geng::Key::A) {
            player_direction.x -= 1.0;
        }

        player.player_direction = player_direction.as_r32();
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);
        self.model.update(delta_time);
    }
}
