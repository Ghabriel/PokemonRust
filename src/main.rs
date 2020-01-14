mod common;
mod config;
mod constants;
mod entities;
mod states;
mod systems;

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
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use crate::{
    config::GameConfig,
    states::LoadingState,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

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
        .with_bundle(AudioBundle::default())?;

    let assets_path = app_root.join("assets");
    Application::build(assets_path, LoadingState::default())?
        .with_resource(game_config)
        .build(game_data)?
        .run();

    Ok(())
}
