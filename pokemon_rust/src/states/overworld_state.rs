use amethyst::{
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    config::GameConfig,
    entities::character::CharacterAnimation,
    events::{EventExecutor, EventQueue},
    states::OverworldAnimationState,
    systems::{
        AnimationSystem,
        CharacterMovementSystem,
        NpcInteractionSystem,
        PlayerInputSystem,
    },
};

use std::{
    cell::RefCell,
    rc::Rc,
    ops::Deref,
};

#[derive(Default)]
pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub event_executor: Rc<RefCell<EventExecutor>>,
}

impl<'a, 'b> OverworldState<'a, 'b> {
    pub fn new(event_executor: Rc<RefCell<EventExecutor>>) -> OverworldState<'a, 'b> {
        OverworldState {
            dispatcher: None,
            event_executor,
        }
    }
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(AnimationSystem::<CharacterAnimation>::new(), "animation_system", &[])
            .with(PlayerInputSystem::new(world), "player_input_system", &[])
            .with(CharacterMovementSystem, "character_movement_system", &["player_input_system"])
            .with(NpcInteractionSystem, "npc_interaction_system", &["player_input_system"])
            .with(FpsCounterSystem, "fps_counter_system", &[])
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

        if world.read_resource::<GameConfig>().show_fps {
            println!("FPS: {}", world.read_resource::<FpsCounter>().sampled_fps());
        }

        {
            let mut event_queue = world.write_resource::<EventQueue>();

            while let Some(event) = event_queue.pop() {
                self.event_executor.borrow_mut().push(event);
            }
        }

        if self.event_executor.borrow().has_new_events() {
            self.event_executor.borrow_mut().start_new_events(world);

            if self.event_executor.borrow().requires_disabled_input() {
                return Trans::Switch(Box::new(OverworldAnimationState::new(self.event_executor.clone())));
            }
        }

        self.event_executor.borrow_mut().tick(world, false);

        Trans::None
    }
}
