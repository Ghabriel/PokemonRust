use amethyst::{
    assets::{Handle, Loader},
    ecs::{World, WorldExt},
    renderer::SpriteSheet,
    ui::{FontHandle, TtfFormat},
};

use super::load_sprite_sheet;

pub struct Resources {
    pub font: FontHandle,
    pub text_box: Handle<SpriteSheet>,
}

pub fn initialise_resources(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "fonts/arial.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let text_box = load_sprite_sheet(world, "sprites/text_box.png", "sprites/text_box.ron");

    world.insert(Resources { font, text_box });
}
