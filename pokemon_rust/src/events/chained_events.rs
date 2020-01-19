use amethyst::ecs::World;

use super::{GameEvent, ShouldDisableInput};

use std::collections::VecDeque;

#[derive(Default)]
pub struct ChainedEvents {
    chain: VecDeque<Box<dyn GameEvent + Sync + Send>>,
    called_start: bool,
}

impl ChainedEvents {
    pub fn add_event(&mut self, event: Box<dyn GameEvent + Sync + Send>) {
        if self.is_complete() {
            self.called_start = false;
        }

        self.chain.push_back(event);
    }
}

impl GameEvent for ChainedEvents {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        self.called_start = true;

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
                self.called_start = false;
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.chain.is_empty()
    }
}
