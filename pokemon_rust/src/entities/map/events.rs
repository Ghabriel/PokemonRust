use super::MapId;

#[derive(Clone, Debug)]
pub enum MapEvent {
    Interaction,
}

#[derive(Clone, Debug)]
pub struct ScriptEvent(pub(super) MapId, pub(super) usize);
