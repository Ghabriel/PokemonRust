mod events;
mod load_map;
mod map;
mod serializable_map;

use amethyst::{
    core::{math::Vector2, Transform},
};

use crate::{
    common::{Direction, get_direction_offset},
    constants::{TILE_SIZE, UNIVERSAL_PLAYER_OFFSET_Y},
    entities::player::Player,
};

use self::map::Map;

use std::collections::HashMap;

pub use self::{
    events::{MapEvent, ScriptEvent},
    load_map::{change_tile, initialise_map, prepare_warp},
    map::{
        GameAction,
        GameActionKind,
        GameScript,
        MapConnection,
        MapScript,
        MapScriptKind,
        Tile,
    },
};

// TODO: find a better name
pub struct MapHandler {
    loaded_maps: HashMap<String, Map>,
    current_map: String,
}

impl MapHandler {
    pub fn get_forward_tile(&self, player: &Player, player_position: &Transform) -> TileData {
        let player_position = PlayerCoordinates(Vector2::new(
            player_position.translation().x,
            player_position.translation().y,
        ));

        let current_map = &self.loaded_maps[&self.current_map];
        let current_tile = current_map.player_to_map_coordinates(&player_position);
        let connection = current_map.connections.get(&current_tile);
        let target_map = if let Some(connection) = connection {
            if connection.directions.contains_key(&player.facing_direction) {
                connection.map.clone()
            } else {
                self.current_map.clone()
            }
        } else {
            self.current_map.clone()
        };

        let (offset_x, offset_y) = get_direction_offset::<f32>(&player.facing_direction);
        let tile_size: f32 = TILE_SIZE.into();
        let position = PlayerCoordinates(Vector2::new(
            player_position.0.x + offset_x * tile_size,
            player_position.0.y + offset_y * tile_size,
        ));

        TileData {
            position,
            map_id: MapId(target_map),
        }
    }

    pub fn is_tile_blocked(&self, tile_data: &TileData) -> bool {
        self.loaded_maps[&tile_data.map_id.0]
            .is_tile_blocked(&tile_data.position)
    }

    pub fn get_action_at(&self, tile_data: &TileData) -> Option<ValidatedGameAction> {
        let map = &self.loaded_maps[&tile_data.map_id.0];
        let tile_coordinates = map.player_to_map_coordinates(&tile_data.position);

        map.actions
            .get(&tile_coordinates)
            .map(|game_action| {
                ValidatedGameAction {
                    when: game_action.when.clone(),
                    script_event: ScriptEvent(tile_data.map_id.clone(), game_action.script_index)
                }
            })
    }

    pub fn get_script_from_event(&self, script_event: &ScriptEvent) -> &GameScript {
        let map = &self.loaded_maps[&(script_event.0).0];

        &map.script_repository[script_event.1]
    }

    pub fn get_map_scripts<'a>(
        &'a self,
        tile_data: &'a TileData,
        kind: MapScriptKind,
    ) -> impl Iterator<Item = ScriptEvent> + 'a {
        self.loaded_maps[&tile_data.map_id.0]
            .map_scripts
            .iter()
            .filter(move |script| script.when == kind)
            .map(move |script| ScriptEvent(tile_data.map_id.clone(), script.script_index))
    }

    pub fn get_current_map_id(&self) -> MapId {
        MapId(self.current_map.clone())
    }

    pub fn get_nearby_connections(
        &self,
        position: &PlayerCoordinates,
    ) -> impl Iterator<Item = (&MapCoordinates, &MapConnection)> {
        let map = &self.loaded_maps[&self.current_map];
        let position = map.player_to_map_coordinates(&position);

        map.connections
            .iter()
            .filter(move |(tile, connection)| {
                let visible_tiles_x = 22;
                let visible_tiles_y = 16;
                // TODO: coordinate system conflict. Bug?
                let distance_x = (tile.0.x as i32) - (position.0.x as i32);
                let distance_y = (tile.0.y as i32) - (position.0.y as i32);
                let leniency = 12;

                connection
                    .directions
                    .iter()
                    .all(|(direction, _)| match direction {
                        Direction::Up | Direction::Down => {
                            distance_y.abs() <= visible_tiles_y / 2 + leniency
                        },
                        Direction::Left | Direction::Right => {
                            distance_x.abs() <= visible_tiles_x / 2 + leniency
                        },
                    })
            })
    }
}

/// A global way to refer to a tile.
pub struct TileData {
    /// The position of the tile.
    pub position: PlayerCoordinates,
    /// The map in which the tile is located.
    pub map_id: MapId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapId(String);

// TODO: find a better name
pub struct ValidatedGameAction {
    pub when: GameActionKind,
    pub script_event: ScriptEvent,
}

/// Represents a position expressed in World Coordinates.
#[derive(Clone)]
pub struct WorldCoordinates(pub Vector2<i32>);

impl Default for WorldCoordinates {
    fn default() -> WorldCoordinates {
        WorldCoordinates(Vector2::new(0, 0))
    }
}

/// Represents a position expressed in Map Coordinates, i.e the position of
/// something relative to the map it's in.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct MapCoordinates(pub Vector2<u32>);

/// Represents a position possibly occupied by a player, expressed in World
/// Coordinates. The universal player offset is included.
#[derive(Clone)]
pub struct PlayerCoordinates(pub Vector2<f32>);

impl PlayerCoordinates {
    pub fn from_world_coordinates(coordinates: &WorldCoordinates) -> PlayerCoordinates {
        PlayerCoordinates(Vector2::new(
            coordinates.0.x as f32,
            coordinates.0.y as f32 + UNIVERSAL_PLAYER_OFFSET_Y,
        ))
    }

    pub fn to_world_coordinates(&self) -> WorldCoordinates {
        WorldCoordinates(Vector2::new(
            self.0.x as i32,
            (self.0.y - UNIVERSAL_PLAYER_OFFSET_Y) as i32,
        ))
    }
}
