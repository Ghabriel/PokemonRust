use amethyst::{
    core::{math::{Point3, Vector3}, Transform},
    ecs::{Entity, world::Builder, World, WorldExt},
    // tiles::{MortonEncoder, Tile, TileMap},
};

use amethyst_tiles::{MortonEncoder, Tile, TileMap};

use super::load_sprite_sheet;

#[derive(Clone, Default)]
pub struct GameTile;

impl Tile for GameTile {
    fn sprite(&self, _coordinates: Point3<u32>, _world: &World) -> Option<usize> {
        Some(0)
    }
}

pub fn initialise_map(world: &mut World) -> Entity {
    let sprite_sheet = load_sprite_sheet(world, "sprites/terrain.png", "sprites/terrain.ron");

    let map = TileMap::<GameTile, MortonEncoder>::new(
        Vector3::new(48, 48, 1),
        Vector3::new(32, 32, 1),
        Some(sprite_sheet),
    );

    world
        .create_entity()
        .with(map)
        .with(Transform::default())
        .build()
}
