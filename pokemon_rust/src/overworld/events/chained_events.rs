//! Generic event. Chains multiple events so that they happen sequentially.

use amethyst::ecs::World;

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

use std::collections::VecDeque;

#[derive(Default)]
pub struct ChainedEvents {
    chain: VecDeque<Box<dyn GameEvent + Sync + Send>>,
    called_start: bool,
}

impl ChainedEvents {
    pub fn add_event(&mut self, event: Box<dyn GameEvent + Sync + Send>) {
        if self.chain.is_empty() {
            self.called_start = false;
        }

        self.chain.push_back(event);
    }
}

impl Clone for ChainedEvents {
    fn clone(&self) -> ChainedEvents {
        ChainedEvents {
            chain: self.chain.iter().map(|event| event.boxed_clone()).collect(),
            called_start: self.called_start,
        }
    }
}

impl GameEvent for ChainedEvents {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        if let Some(event) = self.chain.front() {
            event.get_execution_conditions()
        } else {
            ExecutionConditions::default()
        }
    }

    fn start(&mut self, world: &mut World) {
        self.called_start = true;

        if let Some(event) = self.chain.front_mut() {
            event.start(world);
        }
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        if let Some(event) = self.chain.front_mut() {
            if !self.called_start {
                event.start(world);
                self.called_start = true;
            }

            event.tick(world, disabled_inputs);

            if event.is_complete(world) {
                self.chain.pop_front();
                self.called_start = false;
            }
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        self.chain.is_empty()
    }
}
