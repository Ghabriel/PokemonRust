#![warn(
    clippy::all,
    clippy::pedantic,
)]

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
    clippy::type_complexity,
)]

mod common;
mod config;
mod constants;
mod entities;
mod events;
mod lua;
mod states;
mod systems;

use amethyst::{
    animation::AnimationBundle,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    LoggerConfig,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
        SpriteRender,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::{
    config::GameConfig,
    entities::player::PlayerAnimation,
    states::LoadingState,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");

    let display_config_path = config_path.join("display.ron");
    let game_config = GameConfig::load(config_path.join("settings.ron"));

    let keybindings_config_path = config_path.join("keybindings.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(keybindings_config_path)?
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_bundle(AnimationBundle::<PlayerAnimation, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?;

    let assets_path = app_root.join("assets");
    Application::build(assets_path, LoadingState::default())?
        .with_resource(game_config)
        .build(game_data)?
        .run();

    Ok(())
}
