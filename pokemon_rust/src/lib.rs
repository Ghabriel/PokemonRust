#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::clone_on_copy,
    clippy::cognitive_complexity,
    clippy::filter_map,
    clippy::module_inception,
    clippy::module_name_repetitions,
    clippy::non_ascii_literal,
    clippy::pub_enum_variant_names,
    clippy::single_match_else,
    clippy::trivially_copy_pass_by_ref,
    clippy::type_complexity
)]

pub mod audio;
pub mod common;
pub mod config;
pub mod constants;
pub mod entities;
pub mod events;
pub mod lua;
pub mod map;
pub mod states;
pub mod systems;

use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    start_logger as amethyst_start_logger,
    ui::{RenderUi, UiBundle},
    LoggerConfig,
    Result as AmethystResult,
};

use crate::{config::GameConfig, states::LoadingState};

use std::path::PathBuf;

pub struct PokemonRustParameters {
    pub display_config_path: PathBuf,
    pub game_config_path: PathBuf,
    pub keybindings_config_path: PathBuf,
    pub assets_path: PathBuf,
}

pub fn start_game(params: PokemonRustParameters) -> AmethystResult<()> {
    let PokemonRustParameters {
        display_config_path,
        game_config_path,
        keybindings_config_path,
        assets_path,
    } = params;

    amethyst_start_logger(LoggerConfig::default());

    let game_config = GameConfig::load(game_config_path);

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(keybindings_config_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?;

    Application::build(assets_path, LoadingState::default())?
        .with_resource(game_config)
        .build(game_data)?
        .run();

    Ok(())
}
