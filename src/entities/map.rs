use amethyst::{
    core::Transform,
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use std::collections::HashMap;

use super::load_sprite_sheet;

pub struct Map {
    terrains: Vec<Entity>,
    solids: HashMap<MapPosition, Entity>,
    decorations: HashMap<MapPosition, Entity>,
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

const TILE_SIZE: f32 = 32.;

pub fn initialise_map(world: &mut World) -> Entity {
    let sprite_sheet = load_sprite_sheet(world, "sprites/terrain.png", "sprites/terrain.ron");

    let num_tiles_x: usize = 15;
    let num_tiles_y: usize = 15;

    let mut map = Map {
        terrains: Vec::new(),
        solids: HashMap::new(),
        decorations: HashMap::new(),
        actions: HashMap::new(),
        scripts: Vec::new(),
    };

    map.terrains.reserve(num_tiles_x * num_tiles_y);

    for x in 0..num_tiles_x {
        for y in 0..num_tiles_y {
            let tile = Tile::default();

            let mut transform = Transform::default();
            transform.set_translation_xyz(
                TILE_SIZE * x as f32,
                TILE_SIZE * y as f32,
                -1.,
            );

            let sprite_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: 0,
            };

            let entity = world
                .create_entity()
                .with(tile)
                .with(transform)
                .with(sprite_render)
                .build();

            map.terrains.push(entity);
        }
    }

    world
        .create_entity()
        .with(map)
        .build()
}
