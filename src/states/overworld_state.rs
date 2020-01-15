use amethyst::{
    animation::AnimationBundle,
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder, ReaderId},
    prelude::*,
    renderer::SpriteRender,
    shrev::EventChannel,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    common::{run_script_events, WithBundle},
    entities::{
        event_queue::{EventQueue, GameEvent},
        player::PlayerAnimation,
        map::ScriptEvent,
    },
    states::OverworldTextState,
    systems::{
        MapInteractionSystem,
        PlayerAnimationSystem,
        PlayerInputSystem,
        PlayerMovementSystem,
        StaticPlayerSystem,
    },
};

use std::ops::Deref;

#[derive(Default)]
pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub script_event_reader: Option<ReaderId<ScriptEvent>>,
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        let world = data.world;

        self.script_event_reader = Some(
            world.write_resource::<EventChannel<ScriptEvent>>()
                .register_reader()
        );

        let mut dispatcher = DispatcherBuilder::new()
            .with(MapInteractionSystem::new(world), "map_interaction_system", &[])
            .with(PlayerInputSystem::new(world), "player_input_system", &[])
            .with(PlayerMovementSystem::default(), "player_movement_system", &["player_input_system"])
            .with(StaticPlayerSystem, "static_player_system", &["player_movement_system"])
            .with(PlayerAnimationSystem::new(world), "player_animation_system", &["static_player_system"])
            .with(FpsCounterSystem, "fps_counter_system", &[])
            .with_bundle(world, AnimationBundle::<PlayerAnimation, SpriteRender>::new(
                "sprite_animation_control",
                "sprite_sampler_interpolation",
            )).expect("Failed to build AnimationBundle")
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

        run_script_events(world, self.script_event_reader.as_mut().unwrap());

        // println!("FPS: {}", world.read_resource::<FpsCounter>().sampled_fps());

        let event_queue = world.read_resource::<EventQueue>();

        if let Some(event) = event_queue.front() {
            match event {
                GameEvent::TextEvent(_) => Trans::Switch(Box::new(OverworldTextState::default())),
            }
        } else {
            Trans::None
        }
    }
}
