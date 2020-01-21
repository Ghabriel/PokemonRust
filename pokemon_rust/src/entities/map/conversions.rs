use crate::constants::TILE_SIZE;

use super::coordinates::{CoordinateSystem, MapCoordinates, PlayerCoordinates, WorldCoordinates};

/// Given the position of a tile in Map Coordinates and the reference point of
/// its map, calculates the position of the tile in World Coordinates.
pub fn map_to_world_coordinates(
    tile: &MapCoordinates,
    reference_point: &WorldCoordinates,
) -> WorldCoordinates {
    reference_point.with_offset(&tile.to_world_offset())
}

/// Given the position of a tile in World Coordinates and the reference point of
/// its map, calculates the position of the tile in Map Coordinates.
pub fn world_to_map_coordinates(
    tile: &WorldCoordinates,
    reference_point: &WorldCoordinates,
) -> MapCoordinates {
    let tile_size: u32 = TILE_SIZE.into();

    let scaled_map_coordinates = tile
        .with_offset(&reference_point.to_world_offset().invert())
        .corner();

    MapCoordinates::new(
        scaled_map_coordinates.x() as u32 / tile_size,
        scaled_map_coordinates.y() as u32 / tile_size,
    )
}

/// Given the position of a player and the reference point of its map,
/// calculates the position of its tile in Map Coordinates.
pub fn player_to_map_coordinates(
    player_position: &PlayerCoordinates,
    reference_point: &WorldCoordinates,
) -> MapCoordinates {
    world_to_map_coordinates(
        &player_position.to_world_coordinates(),
        &reference_point
    )
}

/// Given the position of a tile in both Map Coordinates and World Coordinates,
/// calculates the reference point of its map.
pub fn get_reference_point_from_tile(
    tile_map_coordinates: &MapCoordinates,
    tile_world_coordinates: &WorldCoordinates,
) -> WorldCoordinates {
    let offset = tile_map_coordinates
        .to_world_offset()
        .invert();

    tile_world_coordinates.with_offset(&offset)
}
