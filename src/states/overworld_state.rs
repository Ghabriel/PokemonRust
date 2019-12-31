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
    input::{InputEvent, InputHandler, StringBindings},
    prelude::*,
    renderer::{
        Camera,
        SpriteRender,
    },
};

use crate::{
    entities::{
        player::{Direction, PlayerAction, PlayerAnimation, initialise_player, Player},
        map::initialise_map,
    },
    systems::PlayerAnimationSystem,
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

        data.world.register::<Player>();
        data.world.register::<AnimationSet<PlayerAnimation, SpriteRender>>();
        data.world.register::<AnimationControlSet<PlayerAnimation, SpriteRender>>();

        let mut dispatcher_builder = DispatcherBuilder::new()
            // .with(
            //     PrefabLoaderSystemDesc::<MyPrefabData>::default().build(data.world),
            //     "scene_loader",
            //     &[],
            // )
            .with({
                let mut player_storage = data.world.write_storage::<Player>();
                PlayerAnimationSystem::new(&mut player_storage)
            }, "player_movement_system", &[])
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<PlayerAnimation, SpriteRender>::new(
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

        initialise_player(data.world);
        initialise_map(data.world);
        initialise_camera(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        let entities = world.read_resource::<EntitiesRes>();
        let animation_sets = world.read_storage::<AnimationSet<PlayerAnimation, SpriteRender>>();
        let mut control_sets = world.write_storage::<AnimationControlSet<PlayerAnimation, SpriteRender>>();

        for (entity, animation_set) in (&entities, &animation_sets).join() {
            get_animation_set(&mut control_sets, entity)
                .unwrap()
                .add_animation(
                    PlayerAnimation::IdleUp,
                    &animation_set.get(&PlayerAnimation::IdleUp).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::IdleDown,
                    &animation_set.get(&PlayerAnimation::IdleDown).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::IdleLeft,
                    &animation_set.get(&PlayerAnimation::IdleLeft).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::IdleRight,
                    &animation_set.get(&PlayerAnimation::IdleRight).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::WalkUp,
                    &animation_set.get(&PlayerAnimation::WalkUp).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::WalkDown,
                    &animation_set.get(&PlayerAnimation::WalkDown).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::WalkLeft,
                    &animation_set.get(&PlayerAnimation::WalkLeft).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::WalkRight,
                    &animation_set.get(&PlayerAnimation::WalkRight).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::RunUp,
                    &animation_set.get(&PlayerAnimation::RunUp).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::RunDown,
                    &animation_set.get(&PlayerAnimation::RunDown).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::RunLeft,
                    &animation_set.get(&PlayerAnimation::RunLeft).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                )
                .add_animation(
                    PlayerAnimation::RunRight,
                    &animation_set.get(&PlayerAnimation::RunRight).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Init,
                );
        }

        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let world = data.world;

        if let StateEvent::Input(event) = event {
            match event {
                InputEvent::ActionPressed(action) if action == "cancel" => {
                    mutate_player(world, |player| player.action = PlayerAction::Run);
                },
                InputEvent::ActionReleased(action) if action == "cancel" => {
                    mutate_player(world, |player| player.action = PlayerAction::Walk);
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && value < -0.2 => {
                    mutate_player(world, |player| {
                        player.facing_direction = Direction::Down;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && value > 0.2 => {
                    mutate_player(world, |player| {
                        player.facing_direction = Direction::Up;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value: _ } if axis == "vertical" => {
                    let horizontal_value = world
                        .read_resource::<InputHandler<StringBindings>>()
                        .axis_value("horizontal")
                        .unwrap_or(0.);

                    if horizontal_value > -0.2 && horizontal_value < 0.2 {
                        mutate_player(world, |player| player.moving = false);
                    }
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && value < -0.2 => {
                    mutate_player(world, |player| {
                        player.facing_direction = Direction::Left;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && value > 0.2 => {
                    mutate_player(world, |player| {
                        player.facing_direction = Direction::Right;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value: _ } if axis == "horizontal" => {
                    let vertical_value = world
                        .read_resource::<InputHandler<StringBindings>>()
                        .axis_value("vertical")
                        .unwrap_or(0.);

                    if vertical_value > -0.2 && vertical_value < 0.2 {
                        mutate_player(world, |player| player.moving = false);
                    }
                },
                _ => {},
            }
        }

        Trans::None
    }
}

fn mutate_player<F>(world: &mut World, callback: F)
where
    F: Fn(&mut Player) -> ()
{
    let mut players = world.write_storage::<Player>();

    for player in (&mut players).join() {
        callback(player);
    }
}
