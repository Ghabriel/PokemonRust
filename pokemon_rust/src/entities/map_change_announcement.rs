use amethyst::ecs::{Component, DenseVecStorage, Entity};

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

impl Component for MapChangeAnnouncement {
    type Storage = DenseVecStorage<Self>;
}
