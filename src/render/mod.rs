pub mod util;
pub mod world;

use self::world::WorldRender;

use crate::{
    assets::{theme::Theme, Assets},
    model::*,
    prelude::*,
};

use geng_utils::texture as texture_utils;

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
    world: WorldRender,
    theme: Theme,
    world_texture: ugli::Texture,
    fire_texture: ugli::Texture,
    fire_shake: vec2<Coord>,
    unit_quad: ugli::VertexBuffer<draw2d::TexturedVertex>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, theme: Theme) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets, theme.clone()),
            theme,
            fire_texture: texture_utils::new_texture(geng.ugli(), crate::SCREEN_SIZE),
            world_texture: texture_utils::new_texture(geng.ugli(), crate::SCREEN_SIZE),
            fire_shake: vec2::ZERO,
            unit_quad: geng_utils::geometry::unit_quad_geometry(geng.ugli()),
        }
    }

    pub fn draw(&mut self, model: &Model, delta_time: Time, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Update textures' size
        texture_utils::update_texture_size(
            &mut self.world_texture,
            framebuffer.size(),
            self.geng.ugli(),
        );
        texture_utils::update_texture_size(
            &mut self.fire_texture,
            framebuffer.size(),
            self.geng.ugli(),
        );

        // Draw to an intermediate texture for postprocess effects
        let mut world_framebuffer =
            texture_utils::attach_texture(&mut self.world_texture, self.geng.ugli());
        ugli::clear(&mut world_framebuffer, Some(Rgba::BLACK), None, None);

        // Render simplified fire
        self.world.draw_fire(model, &mut world_framebuffer);

        // Fire effect
        let mut fire_framebuffer =
            texture_utils::attach_texture(&mut self.fire_texture, self.geng.ugli());
        ugli::clear(&mut fire_framebuffer, Some(Rgba::BLACK), None, None);

        // Tiled fire texture
        {
            let amplitude = r32(1.0);
            let dir = Angle::from_degrees(r32(thread_rng().gen_range(0.0..360.0)));
            self.fire_shake += dir.unit_vec() * amplitude * delta_time;
        }
        let size = fire_framebuffer.size().as_f32();
        ugli::draw(
            &mut fire_framebuffer,
            &self.assets.shaders.tile_background,
            ugli::DrawMode::TriangleFan,
            &self.unit_quad,
            (
                ugli::uniforms! {
                    u_texture: &self.world_texture,
                    u_fireTexture: &self.assets.sprites.tex_fire,
                    u_camera_pos: &model.camera.center.to_world_f32(),
                    // u_shake: self.fire_shake.as_f32(),
                    u_time: &model.time.as_f32()
                },
                model.camera.uniforms(size),
            ),
            ugli::DrawParameters { ..default() },
        );

        // Background
        ugli::clear(framebuffer, Some(self.theme.background), None, None);

        // Draw the world to screen
        self.world.draw(model, framebuffer);

        // Blend fire with the world
        ugli::draw(
            framebuffer,
            &self.assets.shaders.conv_drunk17,
            ugli::DrawMode::TriangleFan,
            &self.unit_quad,
            ugli::uniforms! {
                u_texture: &self.fire_texture,
                u_resolution: &self.fire_texture.size().as_f32(),
                u_time: &model.time.as_f32()
            },
            ugli::DrawParameters {
                blend_mode: Some(ugli::BlendMode::combined(ugli::ChannelBlendMode {
                    src_factor: ugli::BlendFactor::One,
                    dst_factor: ugli::BlendFactor::One,
                    equation: ugli::BlendEquation::Add,
                })),
                ..default()
            },
        );

        // UI
        self.world.draw_ui(model, framebuffer);
    }

    pub fn draw_ui(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let font = &self.assets.font;
        let framebuffer_size = framebuffer.size().as_f32();

        let time_dead = model.time - model.time_alive;
        if time_dead <= Time::ZERO {
            // In-game ui
            let waves = model.waves.waves.len() + model.waves.infinite_waves_until_boss + 1; // +1 for the boss
            let text = if model.wave_manager.wave_number <= waves {
                format!("Wave {} out of {}", model.wave_manager.wave_number, waves)
            } else {
                format!("Wave {}", model.wave_manager.wave_number)
            };
            font.draw_with_outline(
                framebuffer,
                &geng::PixelPerfectCamera,
                &text,
                vec2::splat(geng::TextAlign::CENTER),
                mat3::translate(framebuffer_size / vec2(2.0, 1.0) - vec2(0.0, 50.0))
                    * mat3::scale_uniform(50.0)
                    * mat3::translate(vec2(0.0, -0.5)),
                self.theme.whiteish,
                0.1,
                self.theme.fire,
            );
        } else {
            let text_color = Color::WHITE;
            let outline_color = Color::BLACK;
            let outline_size = 0.05;

            // Death screen
            font.draw_with_outline(
                framebuffer,
                &geng::PixelPerfectCamera,
                &format!("Time survived: {:.0} sec", model.time_alive.floor()),
                vec2::splat(geng::TextAlign(0.5)),
                mat3::translate(framebuffer_size / 2.0 + vec2(0.0, 50.0))
                    * mat3::scale_uniform(70.0)
                    * mat3::translate(vec2(0.0, -0.5)),
                text_color,
                outline_size,
                outline_color,
            );
            font.draw_with_outline(
                framebuffer,
                &geng::PixelPerfectCamera,
                &format!(
                    "Waves passed: {}",
                    model.wave_manager.wave_number.saturating_sub(1)
                ),
                vec2::splat(geng::TextAlign(0.5)),
                mat3::translate(framebuffer_size / 2.0 + vec2(0.0, -30.0))
                    * mat3::scale_uniform(70.0)
                    * mat3::translate(vec2(0.0, -0.5)),
                text_color,
                outline_size,
                outline_color,
            );
            font.draw_with_outline(
                framebuffer,
                &geng::PixelPerfectCamera,
                "\n[R] to try again",
                vec2::splat(geng::TextAlign(0.5)),
                mat3::translate(framebuffer_size / 2.0 + vec2(0.0, -100.0))
                    * mat3::scale_uniform(70.0)
                    * mat3::translate(vec2(0.0, -0.5)),
                text_color,
                outline_size,
                outline_color,
            );
        }
    }
}
