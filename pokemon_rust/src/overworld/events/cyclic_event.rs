//! Generic event. Repeats an event infinitely and sequentially.

use amethyst::ecs::World;

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

pub struct CyclicEvent {
    prototype: BoxedGameEvent,
    event: BoxedGameEvent,
    called_start: bool,
}

impl CyclicEvent {
    pub fn new(event: BoxedGameEvent) -> CyclicEvent {
        CyclicEvent {
            prototype: event.boxed_clone(),
            event,
            called_start: false,
        }
    }
}

impl Clone for CyclicEvent {
    fn clone(&self) -> CyclicEvent {
        CyclicEvent {
            prototype: self.prototype.boxed_clone(),
            event: self.event.boxed_clone(),
            called_start: self.called_start,
        }
    }
}

impl GameEvent for CyclicEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        self.event.get_execution_conditions()
    }

    fn start(&mut self, world: &mut World) {
        self.event.start(world);
        self.called_start = true;
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        if !self.called_start {
            self.event.start(world);
            self.called_start = true;
        }

        self.event.tick(world, disabled_inputs);

        if self.event.is_complete(world) {
            self.event = self.prototype.boxed_clone();
            self.called_start = false;
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        false
    }
}
