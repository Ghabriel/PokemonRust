use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::constants::TILE_SIZE;

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

use super::load_sprite_sheet;

#[derive(Clone, Debug)]
pub enum MapEvent {
    Interaction,
}

pub struct Map {
    bottom_left_corner: Vector3<i32>,
    num_tiles_x: u32,
    num_tiles_y: u32,
    // terrains: Vec<Tile>,
    terrain_entity: Entity,
    solids: HashMap<Vector2<u32>, Tile>,
    // decorations: HashMap<Vector2<u32>, Tile>,
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

#[derive(Debug)]
pub struct GameAction {
    pub when: GameActionKind,
    pub script_index: usize,
}

#[derive(Debug, Eq, PartialEq)]
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

pub struct MapScript {
    pub when: MapScriptKind,
    pub script_index: usize,
}

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
    let terrain_entity = initialise_terrain_layer(world, "test_map");
    let decoration_entity = initialise_decoration_layer(world, "test_map");

    let mut map = Map {
        bottom_left_corner: Vector3::new(-464, -464, 0),
        num_tiles_x: 29,
        num_tiles_y: 29,
        // terrains: Vec::new(),
        terrain_entity,
        solids: HashMap::new(),
        // decorations: HashMap::new(),
        decoration_entity,
        script_repository: Vec::new(),
        actions: HashMap::new(),
        map_scripts: Vec::new(),
    };

    map.solids.insert(Vector2::new(9, 17), Tile);
    map.solids.insert(Vector2::new(9, 18), Tile);
    map.solids.insert(Vector2::new(10, 17), Tile);
    map.solids.insert(Vector2::new(10, 18), Tile);

    map.script_repository.push(GameScript::Native(|world| {
        use amethyst::shrev::EventChannel;
        use crate::entities::text::TextEvent;

        world
            .write_resource::<EventChannel<TextEvent>>()
            .single_write(TextEvent::new("Hello, world!"));
    }));

    map.actions.insert(Vector2::new(9, 17), GameAction {
        when: GameActionKind::OnInteraction,
        script_index: 0,
    });

    world.insert(map);
}

fn initialise_terrain_layer(world: &mut World, map_name: &str) -> Entity {
    let image_name = format!("maps/{}.png", map_name);
    let ron_name = format!("maps/{}.ron", map_name);

    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, &image_name, &ron_name),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., -1.);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}

fn initialise_decoration_layer(world: &mut World, map_name: &str) -> Entity {
    let image_name = format!("maps/{}_layer3.png", map_name);
    let ron_name = format!("maps/{}.ron", map_name);

    let sprite_render = SpriteRender {
        sprite_sheet: load_sprite_sheet(world, &image_name, &ron_name),
        sprite_number: 0,
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 0.5);

    world
        .create_entity()
        .with(transform)
        .with(sprite_render)
        .build()
}
