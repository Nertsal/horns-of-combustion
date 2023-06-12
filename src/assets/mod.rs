pub mod config;
pub mod controls;
pub mod theme;
pub mod waves;

use geng::prelude::*;

#[derive(geng::asset::Load)]
pub struct Assets {
    pub sprites: SpriteAssets,
    pub shaders: ShaderAssets,
}

#[derive(geng::asset::Load)]
pub struct SpriteAssets {
    #[load(postprocess = "pixel")]
    pub player_human: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub arrow: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub player_barrel: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_default: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_orb: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_small_orb: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_square_snowflake: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_squid_like: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub projectile_wheel_pizza: ugli::Texture,
    #[load(postprocess = "wrap_around")]
    pub tex_fire: ugli::Texture,
}

#[derive(geng::asset::Load)]
pub struct ShaderAssets {
    pub tile_background: ugli::Program,
    pub conv_drunk17: ugli::Program,
    pub health_arc: ugli::Program,
}

/// Use in Assets as `#[load(postprocess = "looping")]`
#[allow(dead_code)]
fn looping(sfx: &mut geng::Sound) {
    sfx.set_looped(true)
}

/// Use in Assets as `#[load(postprocess = "pixel")]`
fn pixel(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}

fn wrap_around(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
    texture.set_wrap_mode(ugli::WrapMode::Repeat);
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"))
            .await
            .context("failed to load assets")
    }
}
