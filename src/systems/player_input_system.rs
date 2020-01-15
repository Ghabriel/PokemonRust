use amethyst::{
    ecs::{
        Entity,
        Read,
        ReaderId,
        ReadStorage,
        System,
        World,
        WorldExt,
        Write,
        WriteStorage,
    },
    input::{InputEvent, InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::{
    common::Direction,
    entities::{
        map::MapEvent,
        player::{PlayerAction, SimulatedPlayer, StaticPlayer},
    },
};

pub struct PlayerInputSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
    player_entity: Entity,
}

impl PlayerInputSystem {
    pub fn new(world: &mut World, player_entity: Entity) -> PlayerInputSystem {
        PlayerInputSystem {
            event_reader: world.write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
            player_entity,
        }
    }
}

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'a, SimulatedPlayer>,
        ReadStorage<'a, StaticPlayer>,
        Write<'a, EventChannel<MapEvent>>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (
        mut simulated_players,
        static_players,
        mut map_event_channel,
        input_event_channel,
        input_handler,
    ): Self::SystemData) {
        let player = &mut simulated_players
            .get_mut(self.player_entity)
            .expect("Failed to retrieve SimulatedPlayer")
            .0;

        for event in input_event_channel.read(&mut self.event_reader) {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    if static_players.contains(self.player_entity) {
                        map_event_channel.single_write(MapEvent::Interaction);
                    }
                },
                InputEvent::ActionPressed(action) if action == "cancel" => {
                    player.action = PlayerAction::Run;
                },
                InputEvent::ActionReleased(action) if action == "cancel" => {
                    player.action = PlayerAction::Walk;
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && *value < -0.2 => {
                    player.facing_direction = Direction::Down;
                    player.moving = true;
                },
                InputEvent::AxisMoved { axis, value } if axis == "vertical" && *value > 0.2 => {
                    player.facing_direction = Direction::Up;
                    player.moving = true;
                },
                InputEvent::AxisMoved { axis, value: _ } if axis == "vertical" => {
                    let horizontal_value = input_handler
                        .axis_value("horizontal")
                        .unwrap_or(0.);

                    if horizontal_value > -0.2 && horizontal_value < 0.2 {
                        player.moving = false;
                    }
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && *value < -0.2 => {
                    player.facing_direction = Direction::Left;
                    player.moving = true;
                },
                InputEvent::AxisMoved { axis, value } if axis == "horizontal" && *value > 0.2 => {
                    player.facing_direction = Direction::Right;
                    player.moving = true;
                },
                InputEvent::AxisMoved { axis, value: _ } if axis == "horizontal" => {
                    let vertical_value = input_handler
                        .axis_value("vertical")
                        .unwrap_or(0.);

                    if vertical_value > -0.2 && vertical_value < 0.2 {
                        player.moving = false;
                    }
                },
                _ => {},
            }
        }
    }
}
