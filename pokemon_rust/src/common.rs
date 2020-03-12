//! Contains common types and functions used throughout the entire game.

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::{World, WorldExt},
    renderer::{
        sprite::{Sprite, TextureCoordinates},
        ImageFormat,
        SpriteSheet,
        SpriteSheetFormat,
        Texture,
    },
    ui::FontHandle,
};

use serde::{Deserialize, Serialize};

/// A wrapper around a progress counter that is inserted into the world as a
/// resource. This allows different parts of the game to use the same progress
/// counter.
pub struct AssetTracker {
    progress_counter: ProgressCounter,
}

impl AssetTracker {
    pub fn new(progress_counter: ProgressCounter) -> AssetTracker {
        AssetTracker { progress_counter }
    }

    pub fn get_progress_counter(&self) -> &ProgressCounter {
        &self.progress_counter
    }

    pub fn get_progress_counter_mut(&mut self) -> &mut ProgressCounter {
        &mut self.progress_counter
    }
}

/// A two-dimensional, four-axis direction enum.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A cache of diverse resources that are used extensively by different parts
/// of the game.
pub struct CommonResources {
    /// The font used by every block of text in the game.
    pub font: FontHandle,
    /// A text box used to wrap text and map change announcements.
    pub text_box: Handle<SpriteSheet>,
    /// A solid black sprite used for screen fading.
    pub black: Handle<SpriteSheet>,
    /// A solid white sprite used for health bars, exp bars, etc.
    pub white: Handle<SpriteSheet>,
    /// A sprite sheet containing the selection arrow displayed in battles.
    pub selection_arrow: Handle<SpriteSheet>,
    /// A sprite sheet containing the "Fight" button displayed in battles.
    pub fight_button: Handle<SpriteSheet>,
    /// A sprite sheet containing the "Run" button displayed in battles.
    pub run_button: Handle<SpriteSheet>,
    /// A sprite sheet containing an HP bar container to be used on the left side.
    pub hp_bar_left: Handle<SpriteSheet>,
    /// A sprite sheet containing an HP bar container to be used on the right side.
    pub hp_bar_right: Handle<SpriteSheet>,
    /// A sprite sheet containing the front side of all gen I Pokémon.
    pub gen1_front: Handle<SpriteSheet>,
    /// A sprite sheet containing the back side of all gen I Pokémon.
    pub gen1_back: Handle<SpriteSheet>,
}

/// Loads a texture + spritesheet from given image/ron filenames, extracting
/// the loader and needed asset storages directly from the world.
pub fn load_sprite_sheet_from_world(
    world: &World,
    image_name: &str,
    ron_name: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let sprite_sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    load_sprite_sheet(
        &loader,
        &texture_storage,
        &sprite_sheet_storage,
        image_name,
        ron_name,
        progress_counter,
    )
}

/// Loads a texture + spritesheet from given image/ron filenames.
pub fn load_sprite_sheet(
    loader: &Loader,
    texture_storage: &AssetStorage<Texture>,
    sprite_sheet_storage: &AssetStorage<SpriteSheet>,
    image_name: &str,
    ron_name: &str,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let texture_handle = loader.load(
        image_name,
        ImageFormat::default(),
        &mut *progress_counter,
        texture_storage,
    );

    loader.load(
        ron_name,
        SpriteSheetFormat(texture_handle),
        &mut *progress_counter,
        sprite_sheet_storage,
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
        sprites: vec![Sprite {
            width: image_size.0 as f32,
            height: image_size.1 as f32,
            offsets: [0., 0.],
            tex_coords: TextureCoordinates {
                left: 0.,
                right: 1.,
                bottom: 1.,
                top: 0.,
            },
        }],
    };

    loader.load_from_data(sprite_sheet, &mut *progress_counter, &world.read_resource())
}

/// Returns a pair of coordinates (x, y) representing a given direction,
/// expressed in a given type.
///
/// # Examples
///
/// ```
/// use pokemon_rust::common::{Direction, get_direction_offset};
///
/// assert_eq!((0, 1), get_direction_offset::<i8>(&Direction::Up));
/// assert_eq!((0, -1), get_direction_offset::<i8>(&Direction::Down));
/// assert_eq!((-1, 0), get_direction_offset::<i8>(&Direction::Left));
/// assert_eq!((1, 0), get_direction_offset::<i8>(&Direction::Right));
/// ```
pub fn get_direction_offset<T>(direction: &Direction) -> (T, T)
where
    T: From<i8>,
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
///
/// # Examples
///
/// ```
/// use pokemon_rust::common::{Direction, get_character_sprite_index_from_direction};
///
/// assert_eq!(0, get_character_sprite_index_from_direction(&Direction::Up));
/// assert_eq!(3, get_character_sprite_index_from_direction(&Direction::Down));
/// assert_eq!(6, get_character_sprite_index_from_direction(&Direction::Left));
/// assert_eq!(9, get_character_sprite_index_from_direction(&Direction::Right));
/// ```
pub fn get_character_sprite_index_from_direction(direction: &Direction) -> usize {
    match direction {
        Direction::Up => 0,
        Direction::Down => 3,
        Direction::Left => 6,
        Direction::Right => 9,
    }
}
