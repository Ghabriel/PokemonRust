use amethyst::{core::math::Vector2, ecs::World};

use super::{ChainedEvents, FadeOutEvent, GameEvent, ShouldDisableInput};

pub struct WarpEvent {
    executor: ChainedEvents,
}

impl WarpEvent {
    pub fn new<T>(map: T, tile: Vector2<u32>) -> WarpEvent
    where
        T: Into<String>
    {
        let mut executor = ChainedEvents::default();
        executor.add_event(Box::new(FadeOutEvent::default()));

        WarpEvent {
            executor,
        }
    }
}

impl GameEvent for WarpEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        self.executor.start(world)
    }

    fn tick(&mut self, world: &mut World, disabled_inputs: bool) {
        self.executor.tick(world, disabled_inputs);
    }

    fn is_complete(&self) -> bool {
        self.executor.is_complete()
    }
}
