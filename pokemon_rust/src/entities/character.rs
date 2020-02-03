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
        npc::NpcAction,
        player::PlayerAction,
    },
    map::{map_to_world_coordinates, MapCoordinates, PlayerCoordinates, TileData, WorldCoordinates},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
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
    /// The action that the character is doing while moving. This determines which
    /// animation to use.
    pub action: CharacterAction,
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

pub enum CharacterAction {
    Player(PlayerAction),
    Npc(NpcAction),
}
