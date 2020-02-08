use amethyst::ecs::Entity;

use crate::common::Direction;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use super::{
    coordinates::WorldCoordinates,
    GameAction,
    GameScript,
    GameScriptParameters,
    MapId,
    MapScript,
};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct SerializableMap {
    pub map_name: String,
    pub base_file_name: String,
    pub layer3_file_name: String,
    pub num_tiles_x: u32,
    pub num_tiles_y: u32,
    pub solids: Vec<(u32, u32)>,
    pub script_repository: Vec<SerializableGameScript>,
    pub actions: HashMap<(u32, u32), GameAction>,
    pub map_scripts: Vec<MapScript>,
    pub connections: HashMap<(u32, u32), SerializableMapConnection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SerializableGameScript {
    Lua {
        file: String,
        function: String,
        parameters: Option<GameScriptParameters>,
    },
}

impl From<SerializableGameScript> for GameScript {
    fn from(script: SerializableGameScript) -> GameScript {
        match script {
            SerializableGameScript::Lua {
                file,
                function,
                parameters,
            } => GameScript::Lua { file, function, parameters },
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SerializableMapConnection {
    pub map: String,
    pub directions: HashMap<Direction, (u32, u32)>,
}

pub(super) struct InitializedMap {
    pub map_id: MapId,
    pub map_name: String,
    pub reference_point: WorldCoordinates,
    pub terrain_entity: Entity,
    pub solids: Vec<(u32, u32)>,
    pub decoration_entity: Entity,
    pub script_repository: Vec<SerializableGameScript>,
    pub actions: HashMap<(u32, u32), GameAction>,
    pub map_scripts: Vec<MapScript>,
    pub connections: HashMap<(u32, u32), SerializableMapConnection>,
}
