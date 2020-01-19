use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
    ecs::{World, WorldExt},
    renderer::SpriteSheet,
    ui::{FontHandle, TtfFormat},
};

use crate::common::load_full_texture_sprite_sheet;

pub struct Resources {
    pub font: FontHandle,
    pub text_box: Handle<SpriteSheet>,
    pub black: Handle<SpriteSheet>,
}

pub fn initialise_resources(world: &mut World, progress_counter: &mut ProgressCounter) {
    let font = world.read_resource::<Loader>().load(
        "fonts/arial.ttf",
        TtfFormat,
        &mut *progress_counter,
        &world.read_resource(),
    );

    let text_box = load_full_texture_sprite_sheet(
        world,
        "sprites/text_box.png",
        (800, 100),
        &mut *progress_counter,
    );

    let black = load_full_texture_sprite_sheet(
        world,
        "sprites/black.png",
        (32, 32),
        &mut *progress_counter,
    );

    world.insert(Resources { font, text_box, black });
}