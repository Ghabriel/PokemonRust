use amethyst::ecs::World;

use super::{GameEvent, ShouldDisableInput};

use std::collections::VecDeque;

pub struct ChainedEvents {
    chain: VecDeque<Box<dyn GameEvent + Sync + Send>>,
    called_start: bool,
}

impl GameEvent for ChainedEvents {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        self.chain
            .front_mut()
            .map(|event| event.start(world))
            .unwrap_or(ShouldDisableInput(false))
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        if let Some(event) = self.chain.front_mut() {
            if !self.called_start {
                event.start(world);
                self.called_start = true;
            }

            event.tick(world, disabled_inputs);

            if event.is_complete() {
                self.chain.pop_front();
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.chain.is_empty()
    }
}
