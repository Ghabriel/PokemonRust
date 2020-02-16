//! Types related to map change announcements.

use amethyst::ecs::Entity;

use std::collections::VecDeque;

/// A queue used to store all pending map change announcements. See
/// [MapChangeAnnouncementSystem](../../systems/struct.MapChangeAnnouncementSystem.html)
/// for more details.
#[derive(Default)]
pub struct MapChangeAnnouncementQueue {
    queue: VecDeque<MapChangeAnnouncement>,
}

impl MapChangeAnnouncementQueue {
    /// Pushes an announcement to this queue.
    pub fn push(&mut self, announcement: MapChangeAnnouncement) {
        self.queue.push_back(announcement);
    }

    /// Removes the first announcement of this queue, if any.
    pub fn pop_front(&mut self) {
        self.queue.pop_front();
    }

    /// Returns a mutable borrow to the first announcement of this queue, if any.
    pub fn front_mut(&mut self) -> Option<&mut MapChangeAnnouncement> {
        self.queue.front_mut()
    }

    /// Checks if this queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of announcements in this queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

/// A component representing a map change announcement. This usually stays
/// inside a `MapChangeAnnouncementQueue` and is managed by the
/// [MapChangeAnnouncementSystem](../../systems/struct.MapChangeAnnouncementSystem.html).
pub struct MapChangeAnnouncement {
    pub elapsed_time: f32,
    pub state: MapChangeAnnouncementState,
    pub box_entity: Entity,
    pub text_entity: Entity,
}

/// The current state of a map change announcement.
pub enum MapChangeAnnouncementState {
    Opening,
    Waiting,
    Closing,
}
