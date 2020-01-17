use amethyst::ecs::World;

use super::{GameEvent, ShouldDisableInput};

pub struct TextEvent {
    text: String,
}

impl TextEvent {
    pub fn new<T>(text: T) -> TextEvent
    where
        T: Into<String>
    {
        TextEvent {
            text: text.into(),
        }
    }
}

impl GameEvent for TextEvent {
    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        // TODO
        ShouldDisableInput(true)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {
        // TODO
        println!("TextEvent::tick");
    }

    fn is_complete(&self) -> bool {
        // TODO
        false
    }
}
