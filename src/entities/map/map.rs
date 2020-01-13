use amethyst::{
    core::{math::{Vector2, Vector3}},
    ecs::{Component, DenseVecStorage, Entity, World},
};

use crate::{
    common::Direction,
    constants::TILE_SIZE,
};

use serde::{Deserialize, Serialize};

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
    pub(super) reference_point: Vector3<i32>,
    pub(super) terrain_entity: Entity,
    pub(super) solids: HashMap<Vector2<u32>, Tile>,
    pub(super) decoration_entity: Entity,
    pub script_repository: Vec<GameScript>,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    pub(super) map_scripts: Vec<MapScript>,
    pub(super) connections: HashMap<Vector2<u32>, MapConnection>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub(super) fn world_to_tile_coordinates(&self, position: &Vector3<f32>) -> Vector2<u32> {
        let position = Vector3::new(
            position.x as i32,
            position.y as i32,
            position.z as i32,
        );
        let tile_size = TILE_SIZE as i32;
        let half_tile = tile_size / 2;
        let target_corner = position - Vector3::new(half_tile, half_tile + 12, 0);
        let normalized_position = (target_corner - self.reference_point) / tile_size;

        Vector2::new(
            normalized_position.x as u32,
            normalized_position.y as u32,
        )
    }

    pub(super) fn is_tile_blocked(&self, position: &Vector3<f32>) -> bool {
        let tile = self.world_to_tile_coordinates(&position);
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
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MapConnection {
    // TODO: maybe pub is ok
    pub(super) map: String,
    pub(super) directions: HashMap<Direction, Vector2<u32>>,
}
