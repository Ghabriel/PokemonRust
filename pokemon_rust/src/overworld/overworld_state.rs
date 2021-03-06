use amethyst::{
    audio::output::init_output,
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    animations::AnimationSystem,
    audio::AudioSystem,
    battle::battle_state::BattleState,
    config::GameConfig,
    overworld::{
        entities::character::CharacterAnimation,
        events::{EventExecutor, EventQueue},
        overworld_animation_state::OverworldAnimationState,
        systems::{
            CharacterMovementSystem,
            MapChangeAnnouncementSystem,
            NpcInteractionSystem,
            PlayerInputSystem,
        },
    },
    text::TextSystem,
};

use std::{cell::RefCell, ops::Deref, rc::Rc};

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
            .with(
                AnimationSystem::<CharacterAnimation>::new(),
                "animation_system",
                &[],
            )
            .with(AudioSystem::default(), "audio_system", &[])
            .with(PlayerInputSystem::new(world), "player_input_system", &[])
            .with(
                CharacterMovementSystem,
                "character_movement_system",
                &["player_input_system"],
            )
            .with(
                NpcInteractionSystem,
                "npc_interaction_system",
                &["player_input_system"],
            )
            .with(MapChangeAnnouncementSystem, "announcement_system", &[])
            .with(TextSystem::new(world), "text_system", &[])
            .with(FpsCounterSystem, "fps_counter_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        init_output(world);
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

            let event_executor = self.event_executor.borrow();

            if event_executor.requires_disabled_input() {
                return Trans::Switch(Box::new(OverworldAnimationState::new(
                    self.event_executor.clone(),
                )));
            } else if event_executor.requires_battle_state() {
                return Trans::Push(Box::new(BattleState::default()));
            }
        }

        self.event_executor.borrow_mut().tick(world, false);

        Trans::None
    }
}
