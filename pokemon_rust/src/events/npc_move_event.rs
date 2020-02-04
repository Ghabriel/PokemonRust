use amethyst::ecs::World;

use super::{CharacterSingleMoveEvent, GameEvent, RepeatedEvent, ShouldDisableInput};

pub struct NpcMoveEvent {
    executor: RepeatedEvent<CharacterSingleMoveEvent>,
}

impl NpcMoveEvent {
    pub fn new(npc_id: usize, num_tiles: usize) -> NpcMoveEvent {
        NpcMoveEvent {
            executor: RepeatedEvent::from_prototype(
                &CharacterSingleMoveEvent::new(npc_id),
                num_tiles,
            ),
        }
    }
}

impl GameEvent for NpcMoveEvent {
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
