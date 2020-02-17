use amethyst::{
    Result as AmethystResult,
    utils::application_root_dir,
};

use pokemon_rust::{PokemonRustParameters, start_game};

fn main() -> AmethystResult<()> {
    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");

    let params = PokemonRustParameters {
        display_config_path: config_path.join("display.ron"),
        game_config_path: config_path.join("settings.ron"),
        keybindings_config_path: config_path.join("keybindings.ron"),
        assets_path: app_root.join("assets"),
    };

    start_game(params)
}
