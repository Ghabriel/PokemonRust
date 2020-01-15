use amethyst::{
    animation::AnimationBundle,
    core::{ArcThreadPool, bundle::SystemBundle},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entity,
        ReaderId,
    },
    prelude::*,
    renderer::SpriteRender,
    shrev::EventChannel,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    common::run_script_events,
    entities::{
        player::{PlayerAnimation, Player},
        map::ScriptEvent,
    },
    systems::{
        MapInteractionSystem,
        PlayerAnimationSystem,
        PlayerInputSystem,
        PlayerMovementSystem,
        StaticPlayerSystem,
        TextSystem,
    },
};

use std::ops::Deref;

pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub player_entity: Entity,
    pub script_event_reader: Option<ReaderId<ScriptEvent>>,
}

impl<'a, 'b> OverworldState<'a, 'b> {
    pub fn new(player_entity: Entity) -> OverworldState<'a, 'b> {
        OverworldState {
            dispatcher: None,
            player_entity,
            script_event_reader: None,
        }
    }
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        let world = data.world;

        self.script_event_reader = Some(
            world.write_resource::<EventChannel<ScriptEvent>>()
                .register_reader()
        );

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(MapInteractionSystem::new(world), "map_interaction_system", &[])
            .with({
                let mut player_storage = world.write_storage::<Player>();
                PlayerAnimationSystem::new(&mut player_storage)
            }, "player_animation_system", &[])
            .with(PlayerInputSystem::new(world, self.player_entity), "player_input_system", &[])
            .with(PlayerMovementSystem::default(), "player_movement_system", &["player_input_system"])
            .with(StaticPlayerSystem, "static_player_system", &["player_movement_system"])
            .with(TextSystem::new(world), "text_system", &[])
            .with(FpsCounterSystem, "fps_counter_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<PlayerAnimation, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ).build(world, &mut dispatcher_builder)
            .expect("Failed to build AnimationBundle");

        let mut dispatcher = dispatcher_builder.build();
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

        Trans::None
    }
}
