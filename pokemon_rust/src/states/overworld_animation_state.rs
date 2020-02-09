use amethyst::{
    core::ArcThreadPool,
    ecs::{
        Dispatcher,
        DispatcherBuilder,
    },
    prelude::*,
};

use crate::{
    entities::character::CharacterAnimation,
    events::{EventExecutor, EventQueue},
    states::OverworldState,
    systems::{
        AnimationSystem,
        CharacterMovementSystem,
        NpcInteractionSystem,
    },
};

use std::{
    cell::RefCell,
    ops::Deref,
    rc::Rc,
};

#[derive(Default)]
pub struct OverworldAnimationState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub event_executor: Rc<RefCell<EventExecutor>>,
}

impl<'a, 'b> OverworldAnimationState<'a, 'b> {
    pub fn new(event_executor: Rc<RefCell<EventExecutor>>) -> OverworldAnimationState<'a, 'b> {
        OverworldAnimationState {
            dispatcher: None,
            event_executor,
        }
    }
}

impl SimpleState for OverworldAnimationState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Entering Animation State");

        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(AnimationSystem::<CharacterAnimation>::new(), "animation_system", &[])
            .with(CharacterMovementSystem, "character_movement_system", &[])
            .with(NpcInteractionSystem, "npc_interaction_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        {
            let mut event_queue = world.write_resource::<EventQueue>();

            while let Some(event) = event_queue.pop() {
                self.event_executor.borrow_mut().push(event);
            }
        }

        if self.event_executor.borrow().has_new_events() {
            let should_disable_input = self.event_executor.borrow_mut().start_new_events(world);

            if !should_disable_input.0 {
                return Trans::Switch(Box::new(OverworldState::new(self.event_executor.clone())));
            }
        }

        if self.event_executor.borrow().is_complete(world) {
            return Trans::Switch(Box::new(OverworldState::new(self.event_executor.clone())));
        }

        self.event_executor.borrow_mut().tick(world, true);

        Trans::None
    }
}
