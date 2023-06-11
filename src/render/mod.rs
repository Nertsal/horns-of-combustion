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
    postprocess_texture: ugli::Texture,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>, theme: Theme) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            world: WorldRender::new(geng, assets, theme),
            postprocess_texture: ugli::Texture::new_with(geng.ugli(), crate::SCREEN_SIZE, |_| {
                Rgba::BLACK
            }),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        // Update postprocess texture size
        if self.postprocess_texture.size() != framebuffer.size() {
            self.postprocess_texture =
                ugli::Texture::new_with(self.geng.ugli(), framebuffer.size(), |_| Rgba::BLACK);
        }

        // Draw to an intermediate texture for postprocess effects
        let mut world_framebuffer = ugli::Framebuffer::new_color(
            self.geng.ugli(),
            ugli::ColorAttachment::Texture(&mut self.postprocess_texture),
        );

        // Render the world
        self.world.draw(model, &mut world_framebuffer);

        // Fire effect
        // TODO
        // ugli::draw(
        //     framebuffer,
        //     &self.assets.shaders.fire,
        //     ugli::DrawMode::TriangleFan,
        //     &unit_geometry(self.geng.ugli()),
        //     ugli::uniforms! {},
        //     ugli::DrawParameters {
        //         blend_mode: Some(ugli::BlendMode::straight_alpha()),
        //         ..default()
        //     },
        // );

        // Draw the final texture to screen
        self.geng.draw2d().textured_quad(
            framebuffer,
            &geng::PixelPerfectCamera,
            Aabb2::ZERO.extend_positive(framebuffer.size().as_f32()),
            &self.postprocess_texture,
            Rgba::WHITE,
        );
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
