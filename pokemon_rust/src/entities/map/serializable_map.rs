use amethyst::core::math::Vector2;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::{GameAction, MapConnection, MapScript};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SerializableMap {
    pub map_name: String,
    pub base_file_name: String,
    pub layer3_file_name: String,
    pub spritesheet_file_name: String,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub solids: Vec<Vector2<u32>>,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    pub map_scripts: Vec<MapScript>,
    pub connections: HashMap<Vector2<u32>, MapConnection>,
}
