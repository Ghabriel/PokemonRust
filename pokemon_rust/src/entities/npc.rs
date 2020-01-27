use amethyst::{
    ecs::{Component, DenseVecStorage},
};

use crate::common::Direction;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Npc {
    pub action: NpcAction,
    pub facing_direction: Direction,
    pub moving: bool,
}

impl Component for Npc {
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
