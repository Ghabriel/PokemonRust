//! Common constants used throughout the entire game.

/// The width of the window, in pixels.
pub const WINDOW_WIDTH: f32 = 800.;

/// The height of the window, in pixels.
pub const WINDOW_HEIGHT: f32 = 600.;

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
pub const HEALTH_BAR_WIDTH: f32 = 200.;

/// The width of the smaller side of health bars, in pixels.
pub const HEALTH_BAR_SMALLER_WIDTH: f32 = 180.;

/// The margin between health bars and the edge of the screen, in pixels.
pub const HEALTH_BAR_MARGIN: f32 = 30.;

/// The horizontal padding between health bars and their contents, in pixels.
pub const HEALTH_BAR_HORIZONTAL_PADDING: f32 = 10.;

/// The vertical padding between health bars and their contents, in pixels.
pub const HEALTH_BAR_VERTICAL_PADDING: f32 = 10.;

/// The height of the health bar of the player and their allies, in pixels.
pub const ALLY_HEALTH_BAR_HEIGHT: f32 = 47.;

/// The height of the health bar of the player's opponents, in pixels.
pub const OPPONENT_HEALTH_BAR_HEIGHT: f32 = 35.;
