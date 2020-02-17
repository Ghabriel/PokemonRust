use amethyst::ecs::{Component, DenseVecStorage, Entity, World};

use crate::{common::Direction, entities::character::CharacterId, events::ScriptEvent};

use serde::{Deserialize, Serialize};

use super::{
    conversions::{map_to_world_coordinates, player_to_map_coordinates},
    serializable_map::InitializedMap,
    MapCoordinates,
    MapId,
    PlayerCoordinates,
    WorldCoordinates,
};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

pub struct Map {
    pub(super) map_id: MapId,
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
            map_id: map.map_id,
            map_name: map.map_name,
            reference_point: map.reference_point,
            terrain_entity: map.terrain_entity,
            solids: map
                .solids
                .into_iter()
                .map(|tile_position| (MapCoordinates::from_tuple(&tile_position), Tile))
                .collect(),
            decoration_entity: map.decoration_entity,
            script_repository: map.script_repository.into_iter().map(Into::into).collect(),
            actions: map
                .actions
                .into_iter()
                .map(|(tile_position, action)| (MapCoordinates::from_tuple(&tile_position), action))
                .collect(),
            map_scripts: map.map_scripts,
            connections: map
                .connections
                .into_iter()
                .map(|(tile_position, connection)| {
                    (
                        MapCoordinates::from_tuple(&tile_position),
                        MapConnection {
                            map: connection.map,
                            directions: connection
                                .directions
                                .into_iter()
                                .map(|(direction, coordinates)| {
                                    (direction, MapCoordinates::from_tuple(&coordinates))
                                })
                                .collect(),
                        },
                    )
                })
                .collect(),
        }
    }

    pub(super) fn map_to_world_coordinates(&self, position: &MapCoordinates) -> WorldCoordinates {
        map_to_world_coordinates(&position, &self.reference_point)
    }

    pub(super) fn player_to_map_coordinates(&self, position: &PlayerCoordinates) -> MapCoordinates {
        player_to_map_coordinates(&position, &self.reference_point)
    }

    pub(super) fn is_tile_blocked(&self, position: &PlayerCoordinates) -> bool {
        let tile = self.player_to_map_coordinates(&position);
        self.solids.contains_key(&tile)
    }

    pub(super) fn get_map_scripts<'a>(
        &'a self,
        kind: MapScriptKind,
    ) -> impl Iterator<Item = ScriptEvent> + 'a {
        self.map_scripts
            .iter()
            .filter(move |script| script.when == kind)
            .map(move |script| ScriptEvent::new(self.map_id.clone(), script.script_index))
    }
}

#[derive(Clone, Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub enum GameScript {
    Native {
        script: fn(&mut World, &Option<GameScriptParameters>) -> (),
        parameters: Option<GameScriptParameters>,
    },
    Lua {
        file: String,
        function: String,
        parameters: Option<GameScriptParameters>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum GameScriptParameters {
    SourceTile(MapCoordinates),
    TargetCharacter(CharacterId),
    SourceMap(String),
}

impl Debug for GameScript {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            GameScript::Native { .. } => write!(formatter, "Native Script"),
            GameScript::Lua {
                file,
                function,
                parameters,
            } => write!(
                formatter,
                "Lua Script({}, {}, {:?})",
                file, function, parameters
            ),
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
    /// Triggered when the map loads.
    OnMapLoad,
    /// Triggered when the player steps on a new tile.
    OnTileChange,
    /// Triggered when the player enters in this map.
    OnMapEnter,
}

#[derive(Clone)]
pub struct MapConnection {
    // TODO: maybe pub is ok
    pub(super) map: String,
    pub(super) directions: HashMap<Direction, MapCoordinates>,
}
