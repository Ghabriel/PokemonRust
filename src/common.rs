use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
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

pub fn load_sprite_sheet(
    world: &World,
    image_name: &str,
    ron_name: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(
        image_name,
        ImageFormat::default(),
        &mut *progress_counter,
        &world.read_resource(),
    );

    loader.load(
        ron_name,
        SpriteSheetFormat(texture_handle),
        &mut *progress_counter,
        &world.read_resource()
    )
}

pub fn get_direction_offset<T>(direction: &Direction) -> (T, T)
where
    T: From<i8>
{
    let (x, y) = match direction {
        Direction::Up => (0, 1),
        Direction::Down => (0, -1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    (x.into(), y.into())
}
