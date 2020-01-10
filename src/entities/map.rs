use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::{
    common::{Direction, load_sprite_sheet},
    constants::TILE_SIZE,
    entities::player::Player,
};

use ron::de::from_reader;

use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    fs::File,
};

#[derive(Clone, Debug)]
pub enum MapEvent {
    Interaction,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializableMap {
    map_name: String,
    base_file_name: String,
    layer3_file_name: String,
    spritesheet_file_name: String,
    num_tiles_x: u32,
    num_tiles_y: u32,
    solids: Vec<Vector2<u32>>,
    actions: HashMap<Vector2<u32>, GameAction>,
    map_scripts: Vec<MapScript>,
    connections: HashMap<Vector2<u32>, MapConnection>,
}

// TODO: find a better name
pub struct MapHandler {
    loaded_maps: HashMap<String, Map>,
    current_map: String,
}
// M.bottom_left_corner += M0.bottom_left_corner;

impl MapHandler {
    pub fn get_forward_tile(
        &self,
        player: &Player,
        player_position: &Transform,
    ) -> TileData {
        let (offset_x, offset_y) = match player.facing_direction {
            Direction::Up => (0., 1.),
            Direction::Down => (0., -1.),
            Direction::Left => (-1., 0.),
            Direction::Right => (1., 0.),
        };

        let tile_size = TILE_SIZE as f32;

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

    pub fn get_action_at(&self, tile_data: &TileData) -> Option<&GameAction> {
        let map = &self.loaded_maps[&tile_data.map_id.0];
        let tile_coordinates = map.world_to_tile_coordinates(&tile_data.position);

        map.actions.get(&tile_coordinates)
    }

    pub fn get_script_from_event(&self, script_event: &ScriptEvent) -> &GameScript {
        let map = &self.loaded_maps[&(script_event.0).0];

        &map.script_repository[script_event.1]
    }
}

pub struct TileData {
    pub position: Vector3<f32>,
    pub map_id: MapId,
}

#[derive(Clone, Debug)]
pub struct MapId(String);

pub struct Map {
    map_name: String,
    bottom_left_corner: Vector3<i32>,
    num_tiles_x: u32,
    num_tiles_y: u32,
    terrain_entity: Entity,
    solids: HashMap<Vector2<u32>, Tile>,
    decoration_entity: Entity,
    pub script_repository: Vec<GameScript>,
    pub actions: HashMap<Vector2<u32>, GameAction>,
    map_scripts: Vec<MapScript>,
    connections: HashMap<Vector2<u32>, MapConnection>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub fn world_to_tile_coordinates(&self, position: &Vector3<f32>) -> Vector2<u32> {
        let position = Vector3::new(
            position.x as i32,
            position.y as i32,
            position.z as i32,
        );
        let tile_size = TILE_SIZE as i32;
        let half_tile = tile_size / 2;
        let target_corner = position - Vector3::new(half_tile, half_tile + 12, 0);
        let normalized_position = (target_corner - self.bottom_left_corner) / tile_size;

        Vector2::new(
            normalized_position.x as u32,
            normalized_position.y as u32,
        )
    }

    pub fn is_tile_blocked(&self, position: &Vector3<f32>) -> bool {
        let tile = self.world_to_tile_coordinates(&position);
        self.solids.contains_key(&tile)
    }
}

#[derive(Clone, Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum MapScriptKind {
    /**
     * Triggered when the player steps on a new tile.
     */
    OnTileChange,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct MapConnection {
    map: String,
    directions: HashMap<Direction, Vector2<u32>>,
}

#[derive(Clone, Debug)]
pub struct ScriptEvent(pub MapId, pub usize);

pub fn initialise_map(world: &mut World) {
    let map_data: SerializableMap = {
        let map_file = application_root_dir()
            .unwrap()
            .join("assets")
            .join("maps")
            .join("test_map")
            .join("map.ron");
        let file = File::open(map_file).expect("Failed opening map file");

        from_reader(file).expect("Failed deserializing map")
    };

    let SerializableMap {
        map_name,
        base_file_name,
        layer3_file_name,
        spritesheet_file_name,
        num_tiles_x,
        num_tiles_y,
        solids,
        actions,
        map_scripts,
        connections,
    } = map_data;

    let terrain_entity = initialise_map_layer(
        world,
        -1.,
        &base_file_name,
        &spritesheet_file_name,
    );
    let decoration_entity = initialise_map_layer(
        world,
        0.5,
        &layer3_file_name,
        &spritesheet_file_name,
    );

    let mut map = Map {
        map_name,
        bottom_left_corner: Vector3::new(
            -(num_tiles_x as i32) * ((TILE_SIZE / 2) as i32),
            -(num_tiles_y as i32) * ((TILE_SIZE / 2) as i32),
            0,
        ),
        num_tiles_x,
        num_tiles_y,
        terrain_entity,
        solids: solids
            .into_iter()
            .map(|tile_position| (tile_position, Tile))
            .collect(),
        decoration_entity,
        script_repository: Vec::new(),
        actions,
        map_scripts,
        connections,
    };

    map.script_repository.push(GameScript::Native(|world| {
        use amethyst::shrev::EventChannel;
        use crate::entities::text::TextEvent;

        world
            .write_resource::<EventChannel<TextEvent>>()
            .single_write(TextEvent::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
    }));

    world.insert(MapHandler {
        loaded_maps: {
            let mut loaded_maps = HashMap::new();
            loaded_maps.insert("test_map".to_string(), map);
            loaded_maps
        },
        current_map: "test_map".to_string(),
    });
}

fn initialise_map_layer(world: &mut World, depth: f32, image_name: &str, ron_name: &str) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, &image_name, &ron_name),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., depth);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}
