use amethyst::{
    animation::AnimationBundle,
    core::{ArcThreadPool, bundle::SystemBundle},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entity,
        ReaderId,
        World,
    },
    input::{InputEvent, InputHandler, StringBindings},
    prelude::*,
    renderer::SpriteRender,
    shrev::EventChannel,
    utils::fps_counter::{FpsCounter, FpsCounterSystem},
};

use crate::{
    common::Direction,
    entities::{
        player::{
            PlayerAction,
            PlayerAnimation,
            Player,
            SimulatedPlayer,
            StaticPlayer,
        },
        map::{GameScript, MapEvent, MapHandler, ScriptEvent},
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

    fn mutate_player<F>(&self, world: &mut World, callback: F)
    where
        F: Fn(&mut Player) -> ()
    {
        let mut players = world.write_storage::<SimulatedPlayer>();
        let player = players
            .get_mut(self.player_entity)
            .expect("Failed to retrieve Player");

        callback(&mut player.0);
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
            .with(PlayerMovementSystem::default(), "player_movement_system", &[])
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
                        .contains(self.player_entity);

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
