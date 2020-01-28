mod conversions;
mod coordinates;
mod load_map;
mod map;
mod serializable_map;

use amethyst::core::Transform;

use crate::{
    common::Direction,
    entities::player::Player,
    events::ScriptEvent,
};

use self::map::Map;

use std::{
    collections::HashMap,
    convert::TryFrom,
};

pub use self::{
    coordinates::{CoordinateSystem, MapCoordinates, PlayerCoordinates, WorldCoordinates},
    conversions::map_to_world_coordinates,
    load_map::{change_tile, initialise_map, prepare_warp},
    map::{
        GameAction,
        GameActionKind,
        GameScript,
        MapConnection,
        LuaGameScriptParameters,
        MapScript,
        MapScriptKind,
        Tile,
    },
};

// TODO: find a better name
pub struct MapHandler {
    loaded_maps: HashMap<String, Map>,
    current_map: MapId,
}

impl MapHandler {
    pub fn get_forward_tile(&self, player: &Player, player_position: &Transform) -> TileData {
        let player_position = PlayerCoordinates::from_transform(&player_position);

        let current_map = &self.loaded_maps[&self.current_map.0];
        let current_tile = current_map.player_to_map_coordinates(&player_position);
        let connection = current_map.connections.get(&current_tile);
        let target_map = if let Some(connection) = connection {
            if connection.directions.contains_key(&player.facing_direction) {
                MapId(connection.map.clone())
            } else {
                self.current_map.clone()
            }
        } else {
            self.current_map.clone()
        };

        TileData {
            position: player_position.offset_by_direction(&player.facing_direction),
            map_id: target_map,
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
                    script_event: ScriptEvent::new(
                        tile_data.map_id.clone(),
                        game_action.script_index,
                    ),
                }
            })
    }

    pub fn get_script(&self, map_id: &MapId, script_index: usize) -> &GameScript {
        let map = &self.loaded_maps[&map_id.0];

        &map.script_repository[script_index]
    }

    pub fn get_map_scripts<'a>(
        &'a self,
        map_id: &'a MapId,
        kind: MapScriptKind,
    ) -> impl Iterator<Item = ScriptEvent> + 'a {
        self.loaded_maps[&map_id.0]
            .map_scripts
            .iter()
            .filter(move |script| script.when == kind)
            .map(move |script| ScriptEvent::new(map_id.clone(), script.script_index))
    }

    pub fn get_current_map_id(&self) -> MapId {
        self.current_map.clone()
    }

    pub fn get_nearby_connections(
        &self,
        position: &PlayerCoordinates,
    ) -> impl Iterator<Item = (&MapCoordinates, &MapConnection)> {
        let map = &self.loaded_maps[&self.current_map.0];
        let position = map.player_to_map_coordinates(&position);

        map.connections
            .iter()
            .filter(move |(tile, connection)| {
                let visible_tiles_x = 22;
                let visible_tiles_y = 16;
                let distance_x = i32::try_from(tile.x()).unwrap() - i32::try_from(position.x()).unwrap();
                let distance_y = i32::try_from(tile.y()).unwrap() - i32::try_from(position.y()).unwrap();
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
