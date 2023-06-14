pub mod config;
pub mod controls;
pub mod theme;
pub mod waves;

use geng::prelude::*;

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
}

#[derive(geng::asset::Load)]
pub struct SpriteAssets {
    pub game_logo: Animation,

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
}

#[derive(geng::asset::Load)]
pub struct ShaderAssets {
    pub tile_background: ugli::Program,
    pub conv_drunk17: ugli::Program,
    pub health_arc: ugli::Program,
}

#[derive(Deref)]
pub struct Animation {
    #[deref]
    pub frames: Vec<(ugli::Texture, f32)>,
}

impl geng::asset::Load for Animation {
    fn load(manager: &geng::asset::Manager, path: &std::path::Path) -> geng::asset::Future<Self> {
        let data = <Vec<u8> as geng::asset::Load>::load(manager, path);
        let manager = manager.clone();
        async move {
            let data = data.await?;
            use image::AnimationDecoder;
            Ok(Self {
                frames: image::codecs::gif::GifDecoder::new(data.as_slice())
                    .unwrap()
                    .into_frames()
                    .map(|frame| {
                        let frame = frame.unwrap();
                        let (n, d) = frame.delay().numer_denom_ms();
                        let mut texture =
                            ugli::Texture::from_image_image(manager.ugli(), frame.into_buffer());
                        pixel(&mut texture);
                        (texture, n as f32 / d as f32 / 1000.0)
                    })
                    .collect(),
            })
        }
        .boxed_local()
    }
    const DEFAULT_EXT: Option<&'static str> = Some("gif");
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

fn load_font(
    manager: &geng::asset::Manager,
    path: &std::path::Path,
) -> geng::asset::Future<Rc<geng::Font>> {
    let manager = manager.clone();
    let path = path.to_owned();
    async move {
        let data = <Vec<u8> as geng::asset::Load>::load(&manager, &path).await?;
        Ok(Rc::new(geng::Font::new(
            manager.ugli(),
            &data,
            geng::font::Options {
                pixel_size: 128.0,
                max_distance: 1.0,
                antialias: false,
            },
        )?))
    }
    .boxed_local()
}
