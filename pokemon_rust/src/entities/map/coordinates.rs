use amethyst::core::{math::Vector2, Transform};

use crate::{
    common::{Direction, get_direction_offset},
    constants::{TILE_SIZE, UNIVERSAL_PLAYER_OFFSET_Y},
};

pub trait CoordinateSystem {
    type CoordinateType;

    fn x(&self) -> Self::CoordinateType;
    fn y(&self) -> Self::CoordinateType;
}

/// Represents an offset expressed in World Coordinates.
pub struct WorldOffset(Vector2<i32>);

impl WorldOffset {
    pub fn new(x: i32, y: i32) -> WorldOffset {
        WorldOffset(Vector2::new(x, y))
    }

    pub fn invert(&self) -> WorldOffset {
        WorldOffset::new(-self.0.x, -self.0.y)
    }
}

/// Represents a position expressed in World Coordinates. It typically
/// refers to the center of a tile.
#[derive(Clone)]
pub struct WorldCoordinates(Vector2<i32>);

impl WorldCoordinates {
    pub fn new(x: i32, y: i32) -> WorldCoordinates {
        WorldCoordinates(Vector2::new(x, y))
    }

    pub fn origin() -> WorldCoordinates {
        WorldCoordinates::new(0, 0)
    }

    pub fn to_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.x() as f32, self.y() as f32, 0.);

        transform
    }

    pub fn offset_by_direction(&self, direction: &Direction) -> WorldCoordinates {
        let tile_size: i32 = TILE_SIZE.into();
        let (offset_x, offset_y) = get_direction_offset::<i32>(&direction);

        WorldCoordinates::new(
            self.x() + tile_size * offset_x,
            self.y() + tile_size * offset_y,
        )
    }

    pub fn with_offset(&self, offset: &WorldOffset) -> WorldCoordinates {
        WorldCoordinates::new(
            self.x() + offset.0.x,
            self.y() + offset.0.y,
        )
    }

    pub fn to_world_offset(&self) -> WorldOffset {
        WorldOffset::new(self.x(), self.y())
    }

    /// Returns the bottom-left corner of the tile that this position refers to.
    pub fn corner(&self) -> WorldCoordinates {
        let half_tile: i32 = (TILE_SIZE / 2).into();

        WorldCoordinates::new(
            self.x() - half_tile,
            self.y() - half_tile,
        )
    }
}

impl Default for WorldCoordinates {
    fn default() -> WorldCoordinates {
        WorldCoordinates(Vector2::new(0, 0))
    }
}

impl CoordinateSystem for WorldCoordinates {
    type CoordinateType = i32;

    fn x(&self) -> i32 {
        self.0.x
    }

    fn y(&self) -> i32 {
        self.0.y
    }
}

/// Represents a position expressed in Map Coordinates, i.e the position of
/// something relative to the map it's in.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct MapCoordinates(Vector2<u32>);

impl MapCoordinates {
    pub fn new(x: u32, y: u32) -> MapCoordinates {
        MapCoordinates(Vector2::new(x, y))
    }

    pub fn from_vector(vector: &Vector2<u32>) -> MapCoordinates {
        MapCoordinates::new(vector.x, vector.y)
    }

    pub fn to_world_offset(&self) -> WorldOffset {
        let tile_size: i32 = TILE_SIZE.into();
        let half_tile = tile_size / 2;

        WorldOffset::new(
            (self.x() as i32) * tile_size + half_tile,
            (self.y() as i32) * tile_size + half_tile,
        )
    }
}

impl CoordinateSystem for MapCoordinates {
    type CoordinateType = u32;

    fn x(&self) -> u32 {
        self.0.x
    }

    fn y(&self) -> u32 {
        self.0.y
    }
}

/// Represents a position possibly occupied by a player, expressed in World
/// Coordinates. The universal player offset is included.
#[derive(Clone)]
pub struct PlayerCoordinates(Vector2<f32>);

impl PlayerCoordinates {
    pub fn new(x: f32, y: f32) -> PlayerCoordinates {
        PlayerCoordinates(Vector2::new(x, y))
    }

    pub fn from_world_coordinates(coordinates: &WorldCoordinates) -> PlayerCoordinates {
        PlayerCoordinates(Vector2::new(
            coordinates.0.x as f32,
            coordinates.0.y as f32 + UNIVERSAL_PLAYER_OFFSET_Y,
        ))
    }

    pub fn to_world_coordinates(&self) -> WorldCoordinates {
        WorldCoordinates(Vector2::new(
            self.0.x as i32,
            (self.0.y - UNIVERSAL_PLAYER_OFFSET_Y) as i32,
        ))
    }

    pub fn from_transform(transform: &Transform) -> PlayerCoordinates {
        PlayerCoordinates::new(
            transform.translation().x,
            transform.translation().y,
        )
    }

    pub fn to_transform(&self) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.x(), self.y(), 0.);

        transform
    }

    pub fn offset_by_direction(&self, direction: &Direction) -> PlayerCoordinates {
        let tile_size: f32 = TILE_SIZE.into();
        let (offset_x, offset_y) = get_direction_offset::<f32>(&direction);

        PlayerCoordinates::new(
            self.x() + tile_size * offset_x,
            self.y() + tile_size * offset_y,
        )
    }
}

impl CoordinateSystem for PlayerCoordinates {
    type CoordinateType = f32;

    fn x(&self) -> f32 {
        self.0.x
    }

    fn y(&self) -> f32 {
        self.0.y
    }
}
