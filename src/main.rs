#[cfg(feature = "dynamic-linking")]
#[allow(unused_imports)]
use dynamic_linking;

mod assets;
mod game;
mod menu;
mod model;
mod render;
mod util;

use geng::prelude::*;

const SCREEN_SIZE: vec2<usize> = vec2(960, 540);

#[derive(clap::Parser, Clone)]
pub struct Opts {
    #[clap(long, default_value = "assets/config.ron")]
    config: std::path::PathBuf,
    #[clap(long, default_value = "assets/level.ron")]
    level: std::path::PathBuf,
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
        title: "Horns of Combustion".to_string(),
        window_size: Some(SCREEN_SIZE),
        ..geng::ContextOptions::from_args(&opts.geng)
    });

    let state = menu::run(&geng, opts);
    geng.run(state)
}
