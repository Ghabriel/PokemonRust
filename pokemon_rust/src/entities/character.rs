use amethyst::{
    assets::{Handle, ProgressCounter},
    ecs::{
        Component,
        DenseVecStorage,
        Entity,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    common::{Direction, get_character_sprite_index_from_direction, load_sprite_sheet},
    config::GameConfig,
    entities::{
        AnimationData,
        AnimationTable,
        CharacterAnimation,
    },
    map::{map_to_world_coordinates, MapCoordinates, PlayerCoordinates, TileData, WorldCoordinates},
};

use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
    pub action: MovementType,
    pub facing_direction: Direction,
    pub next_step: StepKind,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

/// Represents a character movement in progress.
pub struct CharacterMovement {
    /// Stores how much time it will take for the character to reach the target tile.
    pub estimated_time: f32,
    /// Stores the velocity of this movement.
    pub velocity: f32,
    /// The type of this movement. This determines which animation to use.
    pub movement_type: MovementType,
    /// The kind of step the character is doing while moving. This determines at
    /// which point the animation starts.
    pub step_kind: StepKind,
    /// Determines whether processing for this movement has already started.
    pub started: bool,
    /// The source tile.
    pub from: TileData,
    /// The target tile. Must be adjacent to the source tile.
    pub to: TileData,
}

impl Component for CharacterMovement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepKind {
    Left,
    Right,
}

impl StepKind {
    pub fn invert(&mut self) {
        *self = match *self {
            StepKind::Left => StepKind::Right,
            StepKind::Right => StepKind::Left,
        };
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MovementType {
    Walk,
    Run,
}

pub struct AllowedMovements {
    movements: HashMap<MovementType, MovementData>,
}

impl AllowedMovements {
    pub fn can_perform(&self, movement_type: &MovementType) -> bool {
        self.movements.contains_key(movement_type)
    }

    pub fn get_movement_data(&self, movement_type: &MovementType) -> Option<&MovementData> {
        self.movements.get(movement_type)
    }
}

impl Component for AllowedMovements {
    type Storage = DenseVecStorage<Self>;
}

pub struct MovementData {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub velocity: f32,
}
