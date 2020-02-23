#[derive(Debug)]
pub enum BattleEvent {

}

pub struct BattleBackend {
    // TODO
}

impl BattleBackend {
    pub fn tick(&mut self) -> impl Iterator<Item = BattleEvent> {
        return std::iter::empty();
    }
}
