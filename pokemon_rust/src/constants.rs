//! Contains common constants used throughout the entire game.

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
