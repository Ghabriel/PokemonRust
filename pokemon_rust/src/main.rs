use amethyst::{utils::application_root_dir, Result as AmethystResult};

use pokemon_rust::{start_game, PokemonRustParameters};

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
