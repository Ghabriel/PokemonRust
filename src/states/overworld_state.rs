use amethyst::{
    animation::{
        AnimationBundle,
        AnimationCommand,
        AnimationControlSet,
        AnimationSet,
        EndControl,
        get_animation_set,
    },
    core::{ArcThreadPool, bundle::SystemBundle, Parent, Transform},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entity,
        Join,
        ReaderId,
        world::{Builder, EntitiesRes},
        World,
    },
    input::{InputEvent, InputHandler, StringBindings},
    prelude::*,
    renderer::{Camera, SpriteRender},
    shrev::EventChannel,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    common::Direction,
    entities::{
        player::{
            initialise_player,
            PlayerAction,
            PlayerAnimation,
            Player,
            SimulatedPlayer,
            StaticPlayer,
        },
        map::{GameScript, initialise_map, Map, MapEvent, MapHandler, ScriptEvent},
        resources::initialise_resources,
        text::TextEvent,
    },
    systems::{
        MapInteractionSystem,
        PlayerAnimationSystem,
        PlayerMovementSystem,
        StaticPlayerSystem,
        TextSystem,
    },
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
pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub player_entity: Option<Entity>,
    pub script_event_reader: Option<ReaderId<ScriptEvent>>,
}

impl OverworldState<'_, '_> {
    fn mutate_player<F>(&self, world: &mut World, callback: F)
    where
        F: Fn(&mut Player) -> ()
    {
        let mut players = world.write_storage::<SimulatedPlayer>();
        let player = players
            .get_mut(self.player_entity.unwrap())
            .expect("Failed to retrieve Player");

        callback(&mut player.0);
    }
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        data.world.register::<Player>();
        data.world.register::<AnimationSet<PlayerAnimation, SpriteRender>>();
        data.world.register::<AnimationControlSet<PlayerAnimation, SpriteRender>>();
        data.world.register::<Map>();
        data.world.register::<SimulatedPlayer>();

        data.world.insert(EventChannel::<MapEvent>::new());
        data.world.insert(EventChannel::<TextEvent>::new());

        let mut script_event_channel = EventChannel::<ScriptEvent>::new();
        self.script_event_reader = Some(script_event_channel.register_reader());
        data.world.insert(script_event_channel);

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(MapInteractionSystem::new(data.world), "map_interaction_system", &[])
            .with({
                let mut player_storage = data.world.write_storage::<Player>();
                PlayerAnimationSystem::new(&mut player_storage)
            }, "player_animation_system", &[])
            .with(PlayerMovementSystem::default(), "player_movement_system", &[])
            .with(StaticPlayerSystem, "static_player_system", &["player_movement_system"])
            .with(TextSystem::new(data.world), "text_system", &[])
            .with(FpsCounterSystem, "fps_counter_system", &[])
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<PlayerAnimation, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ).build(data.world, &mut dispatcher_builder)
            .expect("Failed to build AnimationBundle");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);

        initialise_resources(data.world);
        let player = initialise_player(data.world);
        initialise_map(data.world);
        initialise_camera(data.world, player);
        self.player_entity = Some(player);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = &mut data.world;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

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

        let mut script_event_reader = self.script_event_reader.as_mut().unwrap();
        let events = world
            .read_resource::<EventChannel<ScriptEvent>>()
            .read(&mut script_event_reader)
            .into_iter()
            .map(Clone::clone)
            .collect::<Vec<ScriptEvent>>();

        for script_event in events {
            let game_script = world
                .read_resource::<MapHandler>()
                .get_script_from_event(&script_event)
                .clone();

            if let GameScript::Native(script) = game_script {
                script(world);
            }
        }

        // println!("FPS: {}", world.read_resource::<FpsCounter>().sampled_fps());

        Trans::None
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        let world = data.world;

        if let StateEvent::Input(event) = event {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    let is_player_static = world
                        .read_storage::<StaticPlayer>()
                        .contains(self.player_entity.unwrap());

                    if is_player_static {
                        world
                            .write_resource::<EventChannel<MapEvent>>()
                            .single_write(MapEvent::Interaction);
                    }
                },
                InputEvent::ActionPressed(action) if action == "cancel" => {
                    self.mutate_player(world, |player| player.action = PlayerAction::Run);
                },
                InputEvent::ActionReleased(action) if action == "cancel" => {
                    self.mutate_player(world, |player| player.action = PlayerAction::Walk);
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && value < -0.2 => {
                    self.mutate_player(world, |player| {
                        player.facing_direction = Direction::Down;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && value > 0.2 => {
                    self.mutate_player(world, |player| {
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
                        self.mutate_player(world, |player| player.moving = false);
                    }
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && value < -0.2 => {
                    self.mutate_player(world, |player| {
                        player.facing_direction = Direction::Left;
                        player.moving = true;
                    });
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && value > 0.2 => {
                    self.mutate_player(world, |player| {
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
                        self.mutate_player(world, |player| player.moving = false);
                    }
                },
                _ => {},
            }
        }

        Trans::None
    }
}
