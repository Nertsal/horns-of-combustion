#![warn(clippy::pedantic)]
#![warn(clippy::todo)]
#![allow(
    clippy::wildcard_imports,
    clippy::module_name_repetitions,
    clippy::uninlined_format_args,
    clippy::semicolon_if_nothing_returned,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::too_many_lines, // TODO: remove
    clippy::needless_pass_by_value,
    clippy::items_after_statements // Used a lot for querying, maybe shouldn't?
)]

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
