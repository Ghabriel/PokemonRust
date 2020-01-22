use amethyst::ecs::{Component, DenseVecStorage, Entity, World};

use crate::common::Direction;

use serde::{Deserialize, Serialize};

use super::{
    conversions::player_to_map_coordinates,
    MapCoordinates,
    PlayerCoordinates,
    serializable_map::InitializedMap,
    WorldCoordinates,
};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

pub struct Map {
    pub(super) map_name: String,
    /**
     * The Reference Point of this map, which corresponds to the coordinates of
     * its bottom-left corner.
     */
    pub(super) reference_point: WorldCoordinates,
    pub(super) terrain_entity: Entity,
    pub(super) solids: HashMap<MapCoordinates, Tile>,
    pub(super) decoration_entity: Entity,
    pub script_repository: Vec<GameScript>,
    pub actions: HashMap<MapCoordinates, GameAction>,
    pub(super) map_scripts: Vec<MapScript>,
    pub(super) connections: HashMap<MapCoordinates, MapConnection>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub(super) fn from_initialized_map(map: InitializedMap) -> Map {
        Map {
            map_name: map.map_name,
            reference_point: map.reference_point,
            terrain_entity: map.terrain_entity,
            solids: map.solids
                .into_iter()
                .map(|tile_position| (MapCoordinates::from_vector(&tile_position), Tile))
                .collect(),
            decoration_entity: map.decoration_entity,
            script_repository: map.script_repository
                .into_iter()
                .map(Into::into)
                .collect(),
            actions: map.actions
                .into_iter()
                .map(|(tile_position, action)| (MapCoordinates::from_vector(&tile_position), action))
                .collect(),
            map_scripts: map.map_scripts,
            connections: map.connections
                .into_iter()
                .map(|(tile_position, connection)| {
                    (
                        MapCoordinates::from_vector(&tile_position),
                        MapConnection {
                            map: connection.map,
                            directions: connection.directions
                                .into_iter()
                                .map(|(direction, coordinates)| {
                                    (direction, MapCoordinates::from_vector(&coordinates))
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }

    pub(super) fn player_to_map_coordinates(&self, position: &PlayerCoordinates) -> MapCoordinates {
        player_to_map_coordinates(&position, &self.reference_point)
    }

    pub(super) fn is_tile_blocked(&self, position: &PlayerCoordinates) -> bool {
        let tile = self.player_to_map_coordinates(&position);
        self.solids.contains_key(&tile)
    }
}

#[derive(Clone, Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub enum GameScript {
    Native(fn(&mut World) -> ()),
    Lua {
        file: String,
        function: String,
    },
}

impl Debug for GameScript {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            GameScript::Native(_) => write!(formatter, "Native Script"),
            GameScript::Lua { file, function } => {
                write!(formatter, "Lua Script({}, {})", file, function)
            },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameAction {
    pub when: GameActionKind,
    pub script_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum GameActionKind {
    /**
     * Triggered when the player presses Z on a tile.
     */
    OnInteraction,
    /**
     * Triggered after the player steps on a tile.
     */
    OnStep,
    /**
     * Triggered when the player tries to step on a tile,
     * _before_ actually stepping on it (e.g doors, sign posts).
     */
    OnStepAttempt,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapScript {
    pub when: MapScriptKind,
    pub script_index: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MapScriptKind {
    /**
     * Triggered when the player steps on a new tile.
     */
    OnTileChange,
    /**
     * Triggered when the player enters in this map.
     */
    OnMapEnter,
}

#[derive(Clone)]
pub struct MapConnection {
    // TODO: maybe pub is ok
    pub(super) map: String,
    pub(super) directions: HashMap<Direction, MapCoordinates>,
}
