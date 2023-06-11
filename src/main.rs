mod assets;
mod game;
mod model;
mod render;
mod util;

use geng::prelude::*;

const SCREEN_SIZE: vec2<usize> = vec2(480, 270);

#[derive(clap::Parser)]
struct Opts {
    #[clap(long, default_value = "assets/config.ron")]
    config: std::path::PathBuf,
    #[clap(long, default_value = "assets/enemies/")]
    enemies: std::path::PathBuf,
    #[clap(long, default_value = "assets/waves.ron")]
    waves: std::path::PathBuf,
    #[clap(long, default_value = "assets/theme.toml")]
    theme: std::path::PathBuf,
    #[clap(long, default_value = "assets/controls.ron")]
    controls: std::path::PathBuf,
    #[clap(flatten)]
    geng: geng::CliArgs,
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let opts: Opts = clap::Parser::parse();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Anlaut Summer Game Jam".to_string(),
        ..geng::ContextOptions::from_args(&opts.geng)
    });

    let future = {
        let geng = geng.clone();
        async move {
            let manager = geng.asset_manager();
            let assets = assets::Assets::load(manager).await.unwrap();
            let config = assets::config::Config::load(&opts.config).await.unwrap();
            let enemies = assets::config::Config::load_enemies(&opts.enemies)
                .await
                .unwrap();
            let waves = assets::waves::WavesConfig::load(&opts.waves).await.unwrap();
            let theme = assets::theme::Theme::load(&opts.theme).await.unwrap();
            let controls = assets::controls::Controls::load(&opts.controls)
                .await
                .unwrap();
            game::Game::new(
                &geng,
                &Rc::new(assets),
                config,
                theme,
                controls,
                enemies,
                waves,
            )
        }
    };
    geng.run_loading(future)
}
