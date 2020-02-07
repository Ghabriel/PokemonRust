use amethyst::ecs::{World, WorldExt};

use crate::{
    common::Direction,
    entities::character::Character,
    map::MapHandler,
};

use super::{BoxedGameEvent, GameEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct CharacterRotateEvent {
    character_id: usize,
    direction: Direction,
}

impl CharacterRotateEvent {
    pub fn new(character_id: usize, direction: Direction) -> CharacterRotateEvent {
        CharacterRotateEvent {
            character_id,
            direction,
        }
    }
}

impl GameEvent for CharacterRotateEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        ShouldDisableInput(false)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_character_by_id(self.character_id);

        world.write_storage::<Character>()
            .get_mut(*entity)
            .unwrap()
            .facing_direction = self.direction.clone();
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
