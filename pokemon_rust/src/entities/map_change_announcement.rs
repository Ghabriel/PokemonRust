use amethyst::ecs::Entity;

use std::collections::VecDeque;

#[derive(Default)]
pub struct MapChangeAnnouncementQueue {
    queue: VecDeque<MapChangeAnnouncement>,
}

impl MapChangeAnnouncementQueue {
    pub fn push(&mut self, announcement: MapChangeAnnouncement) {
        self.queue.push_back(announcement);
    }

    pub fn pop_front(&mut self) {
        self.queue.pop_front();
    }

    pub fn front_mut(&mut self) -> Option<&mut MapChangeAnnouncement> {
        self.queue.front_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

pub struct MapChangeAnnouncement {
    pub elapsed_time: f32,
    pub state: MapChangeAnnouncementState,
    pub box_entity: Entity,
    pub text_entity: Entity,
}

pub enum MapChangeAnnouncementState {
    Opening,
    Waiting,
    Closing,
}
