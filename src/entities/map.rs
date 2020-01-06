use amethyst::{
    core::{math::{Vector2, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::constants::TILE_SIZE;

use std::collections::HashMap;

use super::load_sprite_sheet;

pub struct Map {
    bottom_left_corner: Vector3<f32>,
    num_tiles_x: u32,
    num_tiles_y: u32,
    // terrains: Vec<Tile>,
    terrain_entity: Entity,
    solids: HashMap<Vector2<u32>, Tile>,
    // decorations: HashMap<Vector2<u32>, Tile>,
    decoration_entity: Entity,
    actions: HashMap<Vector2<u32>, GameAction>,
    scripts: Vec<MapScript>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

impl Map {
    pub fn is_tile_blocked(&self, position: &Vector3<f32>) -> bool {
        let tile_offset = (position - Vector3::new(0., 12., 0.)) / (TILE_SIZE as f32);
        let tile_x = (tile_offset.x + ((self.num_tiles_x / 2) as f32)) as u32;
        let tile_y = (tile_offset.y + ((self.num_tiles_y / 2) as f32)) as u32;

        self.solids.contains_key(&Vector2::new(tile_x, tile_y))
    }
}

#[derive(Clone, Default)]
pub struct Tile;

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

/**
 * A position in a map, expressed in tile units.
 */
#[derive(Eq, Hash, PartialEq)]
pub struct MapPosition {
    x: usize,
    y: usize,
}

pub struct GameAction {
    when: GameActionKind,
    // script: TODO,
}

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
    when: MapScriptKind,
    // script: TODO,
}

pub enum MapScriptKind {
    /**
     * Triggered when the player steps on a new tile.
     */
    OnTileChange,
}

pub fn initialise_map(world: &mut World) {
    let terrain_entity = initialise_terrain_layer(world, "test_map");
    let decoration_entity = initialise_decoration_layer(world, "test_map");

    let mut map = Map {
        bottom_left_corner: Vector3::new(-480., -480., 0.),
        num_tiles_x: 29,
        num_tiles_y: 29,
        // terrains: Vec::new(),
        terrain_entity,
        solids: HashMap::new(),
        // decorations: HashMap::new(),
        decoration_entity,
        actions: HashMap::new(),
        scripts: Vec::new(),
    };

    map.solids.insert(Vector2::new(9, 17), Tile);
    map.solids.insert(Vector2::new(9, 18), Tile);
    map.solids.insert(Vector2::new(10, 17), Tile);
    map.solids.insert(Vector2::new(10, 18), Tile);

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
