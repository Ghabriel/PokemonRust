use amethyst::ecs::{Component, DenseVecStorage};

use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    hash::Hash,
};

pub mod npc;
pub mod player;

pub struct AnimationTable<T>
where
    T: 'static + Eq + Hash + Sync + Send
{
    table: HashMap<T, AnimationData>,
    pub active_animation: Option<T>,
    pub timing: f32,
}

impl<T> AnimationTable<T>
where
    T: 'static + Eq + Hash + Sync + Send
{
    pub fn new() -> AnimationTable<T> {
        AnimationTable {
            table: HashMap::new(),
            active_animation: None,
            timing: 0.,
        }
    }

    pub fn get(&self, key: &T) -> Option<&AnimationData> {
        self.table.get(key)
    }

    pub fn insert(&mut self, key: T, value: AnimationData) {
        self.table.insert(key, value);
    }

    pub fn change_animation(&mut self, new_animation: T) {
        self.active_animation = Some(new_animation);
        self.timing = 0.;
    }

    pub fn skip_to_frame_index(&mut self, index: usize) {
        if index == 0 {
            self.timing = 0.;
        } else {
            self.timing = self.table
                .get(self.active_animation.as_ref().unwrap())
                .unwrap()
                .timings[index - 1];
        }
    }
}

impl<T> Component for AnimationTable<T>
where
    T: 'static + Eq + Hash + Sync + Send
{
    type Storage = DenseVecStorage<Self>;
}

pub struct AnimationData {
    pub timings: Vec<f32>,
    pub frames: Vec<usize>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CharacterAnimation {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    NpcMoveUp,
    NpcMoveDown,
    NpcMoveLeft,
    NpcMoveRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    RunUp,
    RunDown,
    RunLeft,
    RunRight,
}
