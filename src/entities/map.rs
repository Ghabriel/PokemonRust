use amethyst::{
    core::{math::{Point3, Vector3}, Transform},
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    // tiles::{MortonEncoder, Tile, TileMap},
};

use amethyst_tiles::{MortonEncoder, Tile, TileMap};

use crate::constants::TILE_SIZE;

use std::collections::HashMap;

use super::load_sprite_sheet;

pub struct Map {
    terrains: Vec<GameTile>,
    solids: HashMap<MapPosition, GameTile>,
    decorations: HashMap<MapPosition, GameTile>,
    actions: HashMap<MapPosition, GameAction>,
    scripts: Vec<MapScript>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Default)]
pub struct GameTile {
    sprite_number: usize,
}

impl Tile for GameTile {
    fn sprite(&self, _coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        Some(self.sprite_number)
    }
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

pub fn initialise_map(world: &mut World) -> Entity {
    let sprite_sheet = load_sprite_sheet(world, "sprites/terrain.png", "sprites/terrain.ron");

    let num_tiles_x: u32 = 48;
    let num_tiles_y: u32 = 48;

    let map = Map {
        terrains: {
            let mut vec = Vec::new();
            vec.resize_with((num_tiles_x * num_tiles_y) as usize, GameTile::default);
            vec
        },
        solids: HashMap::new(),
        decorations: HashMap::new(),
        actions: HashMap::new(),
        scripts: Vec::new(),
    };

    let tile_size = TILE_SIZE as u32;

    let tile_map = TileMap::<GameTile, MortonEncoder>::new(
        Vector3::new(num_tiles_x, num_tiles_y, 1),
        Vector3::new(tile_size, tile_size, 1),
        Some(sprite_sheet),
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(386., 386., 0.);

    world
        .create_entity()
        .with(map)
        .with(tile_map)
        .with(transform)
        .build()
}
