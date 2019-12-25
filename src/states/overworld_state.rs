use amethyst::{
    assets::{Handle, Loader},
    core::Transform,
    ecs::{
        world::Builder,
        World,
    },
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteSheet, SpriteSheetFormat},
};

use crate::{
    entities::player::{Player, initialise_player},
};

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(400., 300., 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(800., 600.))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = loader.load(
        "sprites/player.png",
        ImageFormat::default(),
        (),
        &world.read_resource(),
    );

    loader.load(
        "sprites/player.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource(),
    )
}

#[derive(Default)]
pub struct OverworldState;

impl SimpleState for OverworldState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        data.world.register::<Player>();
        let sprite_sheet = load_sprite_sheet(data.world);
        initialise_player(data.world, sprite_sheet.clone());
        initialise_camera(data.world);
    }
}
