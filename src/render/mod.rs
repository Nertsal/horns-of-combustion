pub mod util;
pub mod world;

use self::world::WorldRender;

use crate::{
    assets::{theme::Theme, Assets},
    model::*,
    util::Vec2RealConversions,
};

use geng::prelude::*;

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
    world: WorldRender,
    theme: Theme,
    world_texture: ugli::Texture,
    fire_texture: ugli::Texture,
    fire_shake: vec2<Coord>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, theme: Theme) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets, theme),
            theme,
            fire_texture: new_texture(geng.ugli(), crate::SCREEN_SIZE),
            world_texture: new_texture(geng.ugli(), crate::SCREEN_SIZE),
            fire_shake: vec2::ZERO,
        }
    }

    pub fn draw(&mut self, model: &Model, delta_time: Time, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);

        // Update textures' size
        update_texture_size(
            &mut self.world_texture,
            self.geng.ugli(),
            framebuffer.size(),
        );
        update_texture_size(&mut self.fire_texture, self.geng.ugli(), framebuffer.size());

        // Draw to an intermediate texture for postprocess effects
        let mut world_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.world_texture),
        );
        ugli::clear(&mut world_framebuffer, Some(Rgba::BLACK), None, None);

        // Render simplified fire
        self.world.draw_fire(model, &mut world_framebuffer);

        // Fire effect
        let mut fire_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.fire_texture),
        );
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
            &unit_geometry(self.geng.ugli()),
            (
                ugli::uniforms! {
                    u_texture: &self.world_texture,
                    u_fireTexture: &self.assets.sprites.tex_fire,
                    u_camera_pos: &model.camera.center.to_world_f32(),
                    u_shake: self.fire_shake.as_f32(),
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
            &unit_geometry(self.geng.ugli()),
            ugli::uniforms! {
                u_texture: &self.fire_texture,
                u_resolution: &self.fire_texture.size().as_f32(),
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
}

fn pixel_perfect_aabb(
    pos: vec2<f32>,
    size: vec2<usize>,
    camera: &impl geng::AbstractCamera2d,
    // screen_size: vec2<f32>,
) -> Aabb2<f32> {
    // Transform to screen space
    let screen_size = crate::SCREEN_SIZE.as_f32();
    let pos = camera_world_to_screen(camera, screen_size, pos);
    let pos = pos.map(|x| x.floor()) + size.as_f32().map(|x| (x / 2.0).fract());
    let screen_aabb = Aabb2::point(pos).extend_symmetric(size.as_f32() / 2.0);
    // Transform back to world
    screen_aabb.map_bounds(|pos| camera.screen_to_world(screen_size, pos))
}

fn camera_world_to_screen(
    camera: &impl geng::AbstractCamera2d,
    framebuffer_size: vec2<f32>,
    pos: vec2<f32>,
) -> vec2<f32> {
    let pos = (camera.projection_matrix(framebuffer_size) * camera.view_matrix()) * pos.extend(1.0);
    let pos = pos.xy() / pos.z;
    // if pos.x.abs() > 1.0 || pos.y.abs() > 1.0 {
    //     return None;
    // }
    vec2(
        (pos.x + 1.0) / 2.0 * framebuffer_size.x,
        (pos.y + 1.0) / 2.0 * framebuffer_size.y,
    )
}

fn unit_geometry(ugli: &Ugli) -> ugli::VertexBuffer<draw2d::TexturedVertex> {
    ugli::VertexBuffer::new_dynamic(ugli, unit_quad().to_vec())
}

fn unit_quad() -> [draw2d::TexturedVertex; 4] {
    [
        draw2d::TexturedVertex {
            a_pos: vec2(-1.0, -1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(0.0, 0.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(1.0, -1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(1.0, 0.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(1.0, 1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(1.0, 1.0),
        },
        draw2d::TexturedVertex {
            a_pos: vec2(-1.0, 1.0),
            a_color: Rgba::WHITE,
            a_vt: vec2(0.0, 1.0),
        },
    ]
}

fn new_texture(ugli: &Ugli, size: vec2<usize>) -> ugli::Texture {
    ugli::Texture::new_with(ugli, size, |_| Rgba::BLACK)
}

fn update_texture_size(texture: &mut ugli::Texture, ugli: &Ugli, size: vec2<usize>) {
    if texture.size() != size {
        *texture = ugli::Texture::new_with(ugli, size, |_| Rgba::BLACK);
    }
}
