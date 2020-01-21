use amethyst::{
    core::math::Vector2,
    ecs::Entity,
};

use crate::common::Direction;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::{coordinates::WorldCoordinates, GameAction, MapScript};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SerializableMap {
    pub map_name: String,
    pub base_file_name: String,
    pub layer3_file_name: String,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub solids: Vec<Vector2<u32>>,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    pub map_scripts: Vec<MapScript>,
    pub connections: HashMap<Vector2<u32>, SerializableMapConnection>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerializableMapConnection {
    pub map: String,
    pub directions: HashMap<Direction, Vector2<u32>>,
}

pub(super) struct InitializedMap {
    pub map_name: String,
    pub reference_point: WorldCoordinates,
    pub terrain_entity: Entity,
    pub solids: Vec<Vector2<u32>>,
    pub decoration_entity: Entity,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    pub map_scripts: Vec<MapScript>,
    pub connections: HashMap<Vector2<u32>, SerializableMapConnection>,
}
