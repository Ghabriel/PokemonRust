use amethyst::{
    animation::{
        AnimationBundle,
        AnimationCommand,
        AnimationControlSet,
        AnimationSet,
        EndControl,
        get_animation_set,
    },
    core::{ArcThreadPool, bundle::SystemBundle, Transform},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Join,
        world::{Builder, EntitiesRes},
        World,
    },
    input::InputEvent,
    prelude::*,
    renderer::{
        Camera,
        SpriteRender,
    },
};

use crate::{
    entities::player::{PlayerAction, initialise_player, Player},
    systems::PlayerMovementSystem,
};

use std::ops::Deref;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(400., 300., 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(800., 600.))
        .with(transform)
        .build();
}

#[derive(Default)]
pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    // pub progress_counter: Option<ProgressCounter>,
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        let mut dispatcher_builder = DispatcherBuilder::new()
            // .with(
            //     PrefabLoaderSystemDesc::<MyPrefabData>::default().build(data.world),
            //     "scene_loader",
            //     &[],
            // )
            .with(PlayerMovementSystem, "player_movement_system", &[])
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<PlayerAction, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ).build(data.world, &mut dispatcher_builder)
            .expect("Failed to build AnimationBundle");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);

        // let mut progress_counter = ProgressCounter::new();
        // let player_prefab = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        //     loader.load(
        //         "sprites/player-walking.ron",
        //         RonFormat,
        //         &mut progress_counter,
        //     )
        // });
        // // Creates new entities with components from MyPrefabData
        // data.world
        //     .create_entity()
        //     .with(player_prefab)
        //     .build();
        // self.progress_counter = Some(progress_counter);

        data.world.register::<Player>();
        data.world.register::<AnimationSet<PlayerAction, SpriteRender>>();
        data.world.register::<AnimationControlSet<PlayerAction, SpriteRender>>();
        initialise_player(data.world);
        initialise_camera(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        let entities = world.read_resource::<EntitiesRes>();
        let animation_sets = world.read_storage::<AnimationSet<PlayerAction, SpriteRender>>();
        let mut control_sets = world.write_storage::<AnimationControlSet<PlayerAction, SpriteRender>>();

        for (entity, animation_set) in (&entities, &animation_sets).join() {
            get_animation_set(&mut control_sets, entity)
                .unwrap()
                .add_animation(
                    PlayerAction::Walk,
                    &animation_set.get(&PlayerAction::Walk).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAction::Run,
                    &animation_set.get(&PlayerAction::Run).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                );
        }

        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Input(event) = event {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    let mut players = data.world.write_storage::<Player>();

                    for player in (&mut players).join() {
                        player.action = PlayerAction::Run;
                        player.temp_flag = true;
                    }
                },
                InputEvent::ActionReleased(action) if action == "action" => {
                    let mut players = data.world.write_storage::<Player>();

                    for player in (&mut players).join() {
                        player.action = PlayerAction::Walk;
                        player.temp_flag = true;
                    }
                },
                InputEvent::ActionPressed(action) if action == "cancel" => {
                    let entities = data.world.read_resource::<EntitiesRes>();
                    let animation_sets = data.world.read_storage::<AnimationSet<PlayerAction, SpriteRender>>();
                    let mut control_sets = data.world.write_storage::<AnimationControlSet<PlayerAction, SpriteRender>>();

                    for (_, _, control_set) in (&entities, &animation_sets, &mut control_sets).join() {
                        println!("{:#?}", control_set);
                    }
                }
                _ => {},
            }
        }

        Trans::None
    }
}
