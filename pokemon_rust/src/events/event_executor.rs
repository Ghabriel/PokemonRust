//! An event executor. This is stored inside each state and is responsible
//! for executing events in parallel while handling the addition of new
//! events, typically coming from the
//! [Event Queue](event_queue/struct.EventQueue.html).

use amethyst::ecs::World;

use super::{GameEvent, ParallelEvents, ShouldDisableInput};

#[derive(Default)]
pub struct EventExecutor {
    root: ParallelEvents,
    incoming_events: Vec<Box<dyn GameEvent + Sync + Send>>,
}

impl EventExecutor {
    pub fn push(&mut self, event: Box<dyn GameEvent + Sync + Send>) {
        self.incoming_events.push(event);
    }

    pub fn has_new_events(&self) -> bool {
        !self.incoming_events.is_empty()
    }

    pub fn start_new_events(&mut self, world: &mut World) -> ShouldDisableInput {
        let mut should_disable_input = false;

        for event in &mut self.incoming_events {
            should_disable_input = should_disable_input || event.start(world).0;
        }

        self.root.add_events(self.incoming_events.drain(..));

        ShouldDisableInput(should_disable_input)
    }

    pub fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.root.tick(world, disabled_inputs);
    }

    pub fn is_complete(&self) -> bool {
        self.root.is_complete()
    }
}
