use super::types::Battle;

#[derive(Debug)]
pub enum BattleEvent {

}

pub struct BattleBackend {
    data: Battle,
}

impl BattleBackend {
    pub fn new(data: Battle) -> BattleBackend {
        BattleBackend {
            data,
        }
    }

    pub fn tick(&mut self) -> impl Iterator<Item = BattleEvent> {
        return std::iter::empty();
    }
}
