use amethyst::{
    animation::{
        AnimationBundle,
        AnimationCommand,
        AnimationControlSet,
        AnimationSet,
        EndControl,
        get_animation_set,
    },
    assets::ProgressCounter,
    core::{ArcThreadPool, bundle::SystemBundle, Parent, Transform},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entity,
        Join,
        world::{Builder, EntitiesRes},
        World,
    },
    prelude::*,
    renderer::{Camera, SpriteRender},
    shrev::EventChannel,
};

use crate::{
    entities::{
        player::{initialise_player, PlayerAnimation, PlayerEntity},
        map::{initialise_map, MapEvent, ScriptEvent},
        resources::initialise_resources,
    },
    events::EventQueue,
    states::OverworldState,
};

use std::ops::Deref;

pub fn initialise_camera(world: &mut World, player: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(800., 600.))
        .with(Parent { entity: player })
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct LoadingState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub progress_counter: Option<ProgressCounter>,
    pub cached_num_loaded_assets: usize,
}

impl SimpleState for LoadingState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading game...");

        let world = data.world;

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<PlayerAnimation, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ).build(world, &mut dispatcher_builder)
            .expect("Failed to build AnimationBundle");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        world.register::<AnimationControlSet<PlayerAnimation, SpriteRender>>();

        world.insert(EventChannel::<MapEvent>::new());
        world.insert(EventChannel::<ScriptEvent>::new());
        world.insert(EventQueue::default());

        let mut progress_counter = ProgressCounter::new();
        initialise_resources(world, &mut progress_counter);
        let player = initialise_player(world, &mut progress_counter);
        initialise_map(world, &mut progress_counter);
        initialise_camera(world, player);
        self.progress_counter = Some(progress_counter);

        world.insert(PlayerEntity(player));

        {
            let entities = world.read_resource::<EntitiesRes>();
            let animation_sets = world.read_storage::<AnimationSet<PlayerAnimation, SpriteRender>>();
            let mut control_sets = world.write_storage::<AnimationControlSet<PlayerAnimation, SpriteRender>>();
            let animations = [
                PlayerAnimation::IdleUp,
                PlayerAnimation::IdleDown,
                PlayerAnimation::IdleLeft,
                PlayerAnimation::IdleRight,
                PlayerAnimation::WalkUp,
                PlayerAnimation::WalkDown,
                PlayerAnimation::WalkLeft,
                PlayerAnimation::WalkRight,
                PlayerAnimation::RunUp,
                PlayerAnimation::RunDown,
                PlayerAnimation::RunLeft,
                PlayerAnimation::RunRight,
            ];

            for (entity, animation_set) in (&entities, &animation_sets).join() {
                let animation_control_set = get_animation_set(&mut control_sets, entity).unwrap();

                for &animation in animations.iter() {
                    animation_control_set.add_animation(
                        animation,
                        &animation_set.get(&animation).unwrap(),
                        EndControl::Loop(None),
                        1.0,
                        AnimationCommand::Init,
                    );
                }
            }
        }
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(data.world);
        }

        if let Some(progress_counter) = &self.progress_counter {
            let num_finished = progress_counter.num_finished();

            if num_finished != self.cached_num_loaded_assets {
                self.cached_num_loaded_assets = num_finished;

                let total = progress_counter.num_assets();
                let percentage = 100 * num_finished / total;
                println!("Loading... {}% ({}/{})", percentage, num_finished, total);
            }
        }

        match &self.progress_counter {
            Some(progress_counter) if progress_counter.is_complete() => {
                Trans::Switch(Box::new(OverworldState::default()))
            },
            _ => Trans::None,
        }
    }
}