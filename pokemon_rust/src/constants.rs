//! Common constants used throughout the entire game.

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
