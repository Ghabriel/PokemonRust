use amethyst::ecs::{Component, DenseVecStorage, Entity, World};

use crate::{
    common::Direction,
    constants::{HALF_TILE_SIZE, TILE_SIZE},
};

use serde::{Deserialize, Serialize};

use super::{CoordinateSystem, MapCoordinates, PlayerCoordinates, WorldCoordinates};

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
    pub(super) fn player_to_map_coordinates(&self, position: &PlayerCoordinates) -> MapCoordinates {
        let tile_size: u32 = TILE_SIZE.into();
        let half_tile: i32 = HALF_TILE_SIZE.into();
        let position = position.to_world_coordinates();

        MapCoordinates::new(
            (position.x() as i32 - half_tile - self.reference_point.x()) as u32 / tile_size,
            (position.y() as i32 - half_tile - self.reference_point.y()) as u32 / tile_size,
        )
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
}

impl Debug for GameScript {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            GameScript::Native(_) => write!(formatter, "Native Script"),
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
