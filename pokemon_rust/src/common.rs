use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
    core::bundle::SystemBundle,
    ecs::{DispatcherBuilder, World, WorldExt},
    error::Error,
    renderer::{
        ImageFormat,
        sprite::{Sprite, TextureCoordinates},
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
    ui::FontHandle,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct CommonResources {
    pub font: FontHandle,
    pub text_box: Handle<SpriteSheet>,
    pub black: Handle<SpriteSheet>,
    pub npc_texture: Handle<Texture>,
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

pub fn load_sprite_sheet_with_texture(
    world: &World,
    texture_handle: Handle<Texture>,
    ron_name: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    loader.load(
        ron_name,
        SpriteSheetFormat(texture_handle),
        &mut *progress_counter,
        &world.read_resource()
    )
}

pub fn load_full_texture_sprite_sheet(
    world: &World,
    image_name: &str,
    image_size: &(u32, u32),
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture = loader.load(
        image_name,
        ImageFormat::default(),
        &mut *progress_counter,
        &world.read_resource(),
    );

    let sprite_sheet = SpriteSheet {
        texture,
        sprites: vec![
            Sprite {
                width: image_size.0 as f32,
                height: image_size.1 as f32,
                offsets: [0., 0.],
                tex_coords: TextureCoordinates {
                    left: 0.,
                    right: 1.,
                    bottom: 1.,
                    top: 0.,
                }
            }
        ]
    };

    loader.load_from_data(
        sprite_sheet,
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

pub fn get_character_sprite_index_from_direction(direction: &Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Down => 3,
        Direction::Left => 6,
        Direction::Right => 9,
    }
}
