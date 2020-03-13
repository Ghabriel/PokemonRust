//! Common constants used throughout the entire game.

/// The width of the window, in pixels.
pub const WINDOW_WIDTH: f32 = 1366.;

/// The height of the window, in pixels.
pub const WINDOW_HEIGHT: f32 = 768.;

/// The size of a tile, in pixels.
pub const TILE_SIZE: u8 = 32;

/// An offset applied to the Y coordinate of the player to better align it
/// to its tile. An offset value of 0 would make it perfectly centered in
/// the tile.
pub const UNIVERSAL_PLAYER_OFFSET_Y: f32 = 12.;

/// The Z coordinate of the Terrain layer of the map.
pub const MAP_TERRAIN_LAYER_Z: f32 = -1.;

/// The Z coordinate of the Decoration layer of the map.
pub const MAP_DECORATION_LAYER_Z: f32 = 0.5;

/// The maximum number of moves that a Pokémon can have.
pub const MOVE_LIMIT: usize = 4;

/// The coordinates of the battle camera.
pub const BATTLE_CAMERA_POSITION: (f32, f32) = (-1000., -1000.);

/// The lowest axis value that is considered an intentional input.
pub const AXIS_SENSITIVITY: f32 = 0.2;

/// The width of health bars, in pixels.
pub const HEALTH_BAR_WIDTH: f32 = 300.;

/// The width of the smaller side of health bars, in pixels.
pub const HEALTH_BAR_SMALLER_WIDTH: f32 = 270.;

/// The margin between health bars and the edge of the screen, in pixels.
pub const HEALTH_BAR_MARGIN: f32 = 30.;

/// The horizontal padding between health bars and their contents, in pixels.
pub const HEALTH_BAR_HORIZONTAL_PADDING: f32 = 10.;

/// The vertical padding between health bars and their contents, in pixels.
pub const HEALTH_BAR_VERTICAL_PADDING: f32 = 10.;

/// The font size of the Pokémon name in a health bar container, in pixels.
pub const HEALTH_BAR_POKEMON_NAME_FONT_SIZE: f32 = 24.;

/// The font size of the Pokémon level in a health bar container, in pixels.
pub const HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE: f32 = 28.;

/// The font size of the Pokémon health text in a health bar container, in pixels.
pub const HEALTH_BAR_POKEMON_HEALTH_TEXT_FONT_SIZE: f32 = 28.;

/// The width of the bar inside a health bar container, in pixels.
pub const BAR_WIDTH: f32 = HEALTH_BAR_SMALLER_WIDTH - 2. * HEALTH_BAR_HORIZONTAL_PADDING;

/// The height of the bar inside a health bar container, in pixels.
pub const BAR_HEIGHT: f32 = 10.;

/// The spacing between the bar and the Pokémon level inside a health bar
/// container, in pixels.
pub const BAR_SPACING: f32 = 2.;

/// The height of the health bar of the player and their allies, in pixels.
pub const ALLY_HEALTH_BAR_HEIGHT: f32 =
    HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE
    + BAR_SPACING
    + BAR_HEIGHT
    + HEALTH_BAR_POKEMON_HEALTH_TEXT_FONT_SIZE;

/// The height of the health bar of the player's opponents, in pixels.
pub const OPPONENT_HEALTH_BAR_HEIGHT: f32 =
    HEALTH_BAR_POKEMON_LEVEL_FONT_SIZE + BAR_SPACING + BAR_HEIGHT + 5.;
