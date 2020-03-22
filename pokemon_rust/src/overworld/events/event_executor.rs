//! An event executor. This is stored inside each state and is responsible
//! for executing events in parallel while handling the addition of new
//! events, typically coming from the
//! [Event Queue](event_queue/struct.EventQueue.html).

use amethyst::ecs::World;

use super::{GameEvent, ParallelEvents};

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

    pub fn requires_disabled_input(&self) -> bool {
        self.root.get_execution_conditions().requires_disabled_input
    }

    pub fn requires_battle_state(&self) -> bool {
        self.root.get_execution_conditions().requires_battle_state
    }

    pub fn start_new_events(&mut self, world: &mut World) {
        for event in &mut self.incoming_events {
            event.start(world);
        }

        self.root.add_events(self.incoming_events.drain(..));
    }

    pub fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.root.tick(world, disabled_inputs);
    }
}
