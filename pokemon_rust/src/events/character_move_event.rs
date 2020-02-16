//! Moves a character forward for a given number of tiles. See the
//! [CharacterMovementSystem](../systems/character_movement_system/struct.CharacterMovementSystem.html)
//! for details on which situations this event can "hang".

use amethyst::ecs::World;

use crate::entities::character::CharacterId;

use super::{BoxedGameEvent, CharacterSingleMoveEvent, ExecutionConditions, GameEvent, RepeatedEvent};

#[derive(Clone)]
pub struct CharacterMoveEvent {
    executor: RepeatedEvent<CharacterSingleMoveEvent>,
}

impl CharacterMoveEvent {
    /// Instantiates a CharacterMoveEvent from a given Character ID and a
    /// number of tiles.
    pub fn new(character_id: CharacterId, num_tiles: usize) -> CharacterMoveEvent {
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

    fn get_execution_conditions(&self) -> ExecutionConditions {
        self.executor.get_execution_conditions()
    }

    fn start(&mut self, world: &mut World) {
        self.executor.start(world);
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.executor.tick(world, disabled_inputs);
    }

    fn is_complete(&self, world: &mut World) -> bool {
        self.executor.is_complete(world)
    }
}
