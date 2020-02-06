use amethyst::ecs::World;

use super::{BoxedGameEvent, CharacterSingleMoveEvent, GameEvent, RepeatedEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct CharacterMoveEvent {
    executor: RepeatedEvent<CharacterSingleMoveEvent>,
}

impl CharacterMoveEvent {
    pub fn new(character_id: usize, num_tiles: usize) -> CharacterMoveEvent {
        CharacterMoveEvent {
            executor: RepeatedEvent::from_prototype(
                &CharacterSingleMoveEvent::new(character_id),
                num_tiles,
            ),
        }
    }
}

impl GameEvent for CharacterMoveEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        self.executor.start(world)
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.executor.tick(world, disabled_inputs);
    }

    fn is_complete(&self, world: &mut World) -> bool {
        self.executor.is_complete(world)
    }
}
