mod events;
mod load_map;
mod map;
mod serializable_map;

use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
};

use crate::{
    common::Direction,
    constants::TILE_SIZE,
    entities::player::Player,
};

use self::map::Map;

use std::collections::HashMap;

pub use self::{
    events::{MapEvent, ScriptEvent},
    load_map::initialise_map,

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
        let (offset_x, offset_y) = match player.facing_direction {
            Direction::Up => (0., 1.),
            Direction::Down => (0., -1.),
            Direction::Left => (-1., 0.),
            Direction::Right => (1., 0.),
        };

        let current_map = &self.loaded_maps[&self.current_map];
        let current_tile = current_map.world_to_tile_coordinates(&player_position.translation());
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

        let tile_size = TILE_SIZE as f32;
        let position = player_position.translation() + Vector3::new(
            offset_x * tile_size,
            offset_y * tile_size,
            0.,
        );

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
        let tile_coordinates = map.world_to_tile_coordinates(&tile_data.position);

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
        position: &Vector3<f32>,
    ) -> impl Iterator<Item = (&Vector2<u32>, &MapConnection)> {
        let map = &self.loaded_maps[&self.current_map];
        let position = map.world_to_tile_coordinates(&position);

        map.connections
            .iter()
            .filter(move |(tile, connection)| {
                let visible_tiles_x = 22;
                let visible_tiles_y = 16;
                let distance_x = (tile.x as i32) - (position.x as i32);
                let distance_y = (tile.y as i32) - (position.y as i32);
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

pub struct TileData {
    pub position: Vector3<f32>,
    pub map_id: MapId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MapId(String);

// TODO: find a better name
pub struct ValidatedGameAction {
    pub when: GameActionKind,
    pub script_event: ScriptEvent,
}


