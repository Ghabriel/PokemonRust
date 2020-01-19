use super::GameEvent;

use std::collections::VecDeque;

#[derive(Default)]
pub struct EventQueue {
    events: VecDeque<Box<dyn GameEvent + Sync + Send>>,
}

impl EventQueue {
    pub fn push<T>(&mut self, event: T)
    where
        T: GameEvent + Sync + Send + 'static
    {
        self.events.push_back(Box::new(event));
    }

    pub fn pop(&mut self) -> Option<Box<dyn GameEvent + Sync + Send>> {
        self.events.pop_front()
    }
}
