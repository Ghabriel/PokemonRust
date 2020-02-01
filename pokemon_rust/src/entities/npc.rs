use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::{
    common::Direction,
    map::TileData,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Npc {
    pub action: NpcAction,
    pub facing_direction: Direction,
    pub moving: bool,
    pub kind: String,
}

impl Component for Npc {
    type Storage = DenseVecStorage<Self>;
}

/// Represents an NPC movement in progress.
pub struct NpcMovement {
    /// Stores how much time it will take for the NPC to reach the target tile.
    pub estimated_time: f32,
    /// Determines whether processing for this movement has already started.
    pub started: bool,
    /// The source tile.
    pub from: TileData,
    /// The target tile. Must be adjacent to the source tile.
    pub to: TileData,
}

impl Component for NpcMovement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NpcAction {
    Idle,
    Moving,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NpcAnimation {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
}
