use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::constants::TILE_SIZE;

use std::collections::HashMap;

use super::load_sprite_sheet;

pub struct Map {
    terrains: Vec<Tile>,
    solids: HashMap<MapPosition, Tile>,
    decorations: HashMap<MapPosition, Tile>,
    actions: HashMap<MapPosition, GameAction>,
    scripts: Vec<MapScript>,
}

impl Component for Map {
    type Storage = DenseVecStorage<Self>;
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

pub fn initialise_map(world: &mut World) -> Entity {
    let num_tiles_x: u32 = 48;
    let num_tiles_y: u32 = 48;

    let map = Map {
        terrains: {
            let mut vec = Vec::new();
            vec.resize_with((num_tiles_x * num_tiles_y) as usize, Tile::default);
            vec
        },
        solids: HashMap::new(),
        decorations: HashMap::new(),
        actions: HashMap::new(),
        scripts: Vec::new(),
    };

    let sprite_sheet = load_sprite_sheet(world, "test_map.png", "test_map.ron");

    let sprite_render = SpriteRender {
        sprite_sheet,
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
