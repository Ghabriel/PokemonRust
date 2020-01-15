use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct EventQueue {
    events: VecDeque<GameEvent>,
}

impl EventQueue {
    pub fn front(&self) -> Option<&GameEvent> {
        self.events.front()
    }

    pub fn push(&mut self, event: GameEvent) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<GameEvent> {
        self.events.pop_front()
    }
}

#[derive(Debug)]
pub enum GameEvent {
    TextEvent(String),
}
