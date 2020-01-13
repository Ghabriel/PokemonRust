use amethyst::{
    assets::{Handle, Loader},
    ecs::{World, WorldExt},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn load_sprite_sheet(world: &World, image_name: &str, ron_name: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(image_name, ImageFormat::default(), (), &world.read_resource());

    loader.load(ron_name, SpriteSheetFormat(texture_handle), (), &world.read_resource())
}
