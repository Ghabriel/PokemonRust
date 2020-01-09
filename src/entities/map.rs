use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
    utils::application_root_dir,
};

use crate::constants::TILE_SIZE;

use ron::de::from_reader;

use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    fs::File,
};

use super::load_sprite_sheet;

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
    bottom_left_corner: Vector3<i32>,
    num_tiles_x: u32,
    num_tiles_y: u32,
    solids: Vec<Vector2<u32>>,
    actions: HashMap<Vector2<u32>, GameAction>,
    map_scripts: Vec<MapScript>,
}

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

#[derive(Clone, Debug)]
pub struct ScriptEvent(pub usize);

pub fn initialise_map(world: &mut World) {
    let map_data: SerializableMap = {
        let map_file = application_root_dir()
            .unwrap()
            .join("assets")
            .join("maps")
            .join("test_map_data.ron");
        let file = File::open(map_file).expect("Failed opening map file");

        from_reader(file).expect("Failed deserializing map")
    };

    let SerializableMap {
        map_name,
        base_file_name,
        layer3_file_name,
        spritesheet_file_name,
        bottom_left_corner,
        num_tiles_x,
        num_tiles_y,
        solids,
        actions,
        map_scripts,
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
        bottom_left_corner,
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
    };

    map.script_repository.push(GameScript::Native(|world| {
        use amethyst::shrev::EventChannel;
        use crate::entities::text::TextEvent;

        world
            .write_resource::<EventChannel<TextEvent>>()
            .single_write(TextEvent::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."));
    }));

    world.insert(map);
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
