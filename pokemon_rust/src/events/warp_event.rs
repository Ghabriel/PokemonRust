use amethyst::{
    core::math::Vector2,
    ecs::World,
};

use super::{GameEvent, ShouldDisableInput};

pub struct WarpEvent {
    map: String,
    tile: Vector2<u32>,
}

impl WarpEvent {
    pub fn new<T>(map: T, tile: Vector2<u32>) -> WarpEvent
    where
        T: Into<String>
    {
        WarpEvent {
            map: map.into(),
            tile,
        }
    }
}

impl GameEvent for WarpEvent {
    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        // TODO
        ShouldDisableInput(true)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {
        // TODO
        println!("WarpEvent::tick");
    }

    fn is_complete(&self) -> bool {
        // TODO
        false
    }
}
