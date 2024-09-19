pub mod config;
pub mod controls;
pub mod theme;
pub mod waves;

use geng_utils::gif::GifFrame;

use crate::prelude::*;

#[derive(geng::asset::Load)]
pub struct Assets {
    pub sprites: SpriteAssets,
    pub shaders: ShaderAssets,
    pub sounds: SoundAssets,
    #[load(load_with = "load_font(&manager, &base_path.join(\"fonts/avalancheno.ttf\"))")]
    pub font: Rc<geng::Font>,
}

#[derive(geng::asset::Load)]
pub struct SoundAssets {
    pub shoot: geng::Sound,
    pub explosion: geng::Sound,
    #[load(postprocess = "looping", path = "music.mp3")]
    pub music: geng::Sound,
}

#[derive(geng::asset::Load)]
pub struct SpriteAssets {
    #[load(load_with = "load_gif(&manager, &base_path.join(\"game_logo.gif\"))")]
    pub game_logo: Vec<GifFrame>,

    #[load(postprocess = "pixel")]
    pub barrel: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub player_human: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub arrow: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub player_barrel: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub crosshair: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub crosshair_barrel: ugli::Texture,

    // Projectiles sprites
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

    // Shader textures
    #[load(postprocess = "wrap_around")]
    pub tex_fire: ugli::Texture,

    // Enemy sprites
    #[load(postprocess = "pixel")]
    pub enemy_clown: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub enemy_death_star: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub enemy_dice: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub enemy_huge: ugli::Texture,

    // Boss sprites
    #[load(postprocess = "pixel")]
    pub boss_foot: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub boss_leg: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub boss_body: ugli::Texture,
    #[load(postprocess = "pixel")]
    pub boss_eye: ugli::Texture,
}

#[derive(geng::asset::Load)]
pub struct ShaderAssets {
    pub tile_background: ugli::Program,
    pub conv_drunk17: ugli::Program,
    pub health_arc: ugli::Program,
}

/// Use in Assets as `#[load(postprocess = "looping")]`
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
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}

fn load_gif(
    manager: &geng::asset::Manager,
    path: &std::path::Path,
) -> geng::asset::Future<Vec<GifFrame>> {
    let manager = manager.clone();
    let path = path.to_owned();
    async move {
        geng_utils::gif::load_gif(
            &manager,
            &path,
            geng_utils::gif::GifOptions {
                frame: geng::asset::TextureOptions {
                    filter: ugli::Filter::Nearest,
                    ..Default::default()
                },
            },
        )
        .await
    }
    .boxed_local()
}

fn load_font(
    manager: &geng::asset::Manager,
    path: &std::path::Path,
) -> geng::asset::Future<Rc<geng::Font>> {
    let manager = manager.clone();
    let path = path.to_owned();
    async move {
        let font = <geng::Font as geng::asset::Load>::load(
            &manager,
            &path,
            &geng::font::Options {
                pixel_size: 128.0,
                max_distance: 1.0,
                antialias: false,
                distance_mode: geng::font::DistanceMode::Euclid,
            },
        )
        .await?;
        Ok(Rc::new(font))
    }
    .boxed_local()
}
