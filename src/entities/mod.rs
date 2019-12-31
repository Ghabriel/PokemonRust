pub mod map;
pub mod player;

use amethyst::{
    assets::{Handle, Loader},
    ecs::{World, WorldExt},
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat},
};

pub fn load_sprite_sheet(world: &World, image_name: &str, ron_name: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(image_name, ImageFormat::default(), (), &world.read_resource());

    loader.load(ron_name, SpriteSheetFormat(texture_handle), (), &world.read_resource())
}
