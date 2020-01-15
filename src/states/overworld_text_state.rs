use amethyst::{
    core::ArcThreadPool,
    ecs::{
        Dispatcher,
        DispatcherBuilder,
    },
    prelude::*,
    shrev::EventChannel,
};

use crate::{
    entities::{
        event_queue::{EventQueue, GameEvent},
        text::TextEvent,
    },
    states::OverworldState,
    systems::text_system::{EventStatus, TextSystem},
};

use std::ops::Deref;

#[derive(Default)]
pub struct OverworldTextState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl SimpleState for OverworldTextState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Entering Text State");

        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(TextSystem::new(world), "text_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        let mut event_queue = world.write_resource::<EventQueue>();

        if let Some(GameEvent::TextEvent(_)) = event_queue.front() {
            if let Some(GameEvent::TextEvent(text)) = event_queue.pop() {
                world.write_resource::<EventChannel<TextEvent>>()
                    .single_write(TextEvent::new(text));

                Trans::None
            } else {
                unreachable!();
            }
        } else if !world.read_resource::<EventStatus>().ended {
            Trans::None
        } else {
            Trans::Switch(Box::new(OverworldState::default()))
        }
    }
}
