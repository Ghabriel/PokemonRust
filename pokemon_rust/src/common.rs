//! Contains common types and functions used throughout the entire game.

use amethyst::{
    assets::{Handle, Loader, ProgressCounter},
    ecs::{World, WorldExt},
    renderer::{
        ImageFormat,
        sprite::{Sprite, TextureCoordinates},
        SpriteSheet,
        SpriteSheetFormat,
    },
    ui::FontHandle,
};

use serde::{Deserialize, Serialize};

/// A two-dimensional, four-axis direction enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A group of diverse resources that are used extensively by different parts
/// of the game.
pub struct CommonResources {
    pub font: FontHandle,
    pub text_box: Handle<SpriteSheet>,
    pub black: Handle<SpriteSheet>,
}

/// Loads a texture + spritesheet from given image/ron filenames.
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

/// Loads a texture from a given image filename, and creates a spritesheet to
/// it representing the entire texture.
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

/// Returns a pair of coordinates (x, y) representing a given direction,
/// expressed in a given type.
///
/// # Examples
///
/// ```
/// use pokemon_rust::common::get_direction_offset;
///
/// assert_eq!((0, 1), get_direction_offset::<i8>(Direction::Up));
/// assert_eq!((0, -1), get_direction_offset::<i8>(Direction::Down));
/// assert_eq!((-1, 0), get_direction_offset::<i8>(Direction::Left));
/// assert_eq!((1, 0), get_direction_offset::<i8>(Direction::Right));
/// ```
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

/// Returns the appropriate sprite index to use for a standing character that
/// is facing a given direction.
pub fn get_character_sprite_index_from_direction(direction: &Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Down => 3,
        Direction::Left => 6,
        Direction::Right => 9,
    }
}
