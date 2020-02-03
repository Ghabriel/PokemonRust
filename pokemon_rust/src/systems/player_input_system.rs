use amethyst::{
    ecs::{
        Read,
        ReaderId,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteStorage,
        World,
        WorldExt,
    },
    input::{InputEvent, InputHandler, StringBindings},
    shrev::EventChannel,
};

use crate::{
    common::Direction,
    entities::{
        character::Character,
        player::{Player, PlayerAction, PlayerEntity, PlayerMovement},
    },
    events::{EventQueue, MapInteractionEvent, PlayerSingleMoveEvent},
};

pub struct PlayerInputSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl PlayerInputSystem {
    pub fn new(world: &mut World) -> PlayerInputSystem {
        PlayerInputSystem {
            event_reader: world.write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        }
    }
}

impl<'a> System<'a> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, PlayerMovement>,
        Write<'a, EventQueue>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, PlayerEntity>,
    );

    fn run(&mut self, (
        mut characters,
        mut players,
        movements,
        mut event_queue,
        input_event_channel,
        input_handler,
        player_entity,
    ): Self::SystemData) {
        if movements.contains(player_entity.0) {
            // Ignores all incoming events
            for _ in input_event_channel.read(&mut self.event_reader) { }
            return;
        }

        for event in input_event_channel.read(&mut self.event_reader) {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    event_queue.push(MapInteractionEvent);
                },
                _ => {},
            }
        }

        let character = &mut characters
            .get_mut(player_entity.0)
            .expect("Failed to retrieve Character");

        let player = &mut players
            .get_mut(player_entity.0)
            .expect("Failed to retrieve Player");

        if input_handler.action_is_down("cancel").unwrap_or(false) {
            player.action = PlayerAction::Run;
        } else {
            player.action = PlayerAction::Walk;
        }

        let horizontal_value = input_handler
            .axis_value("horizontal")
            .unwrap_or(0.);

        if horizontal_value < -0.2 {
            character.facing_direction = Direction::Left;
            event_queue.push(PlayerSingleMoveEvent);
        } else if horizontal_value > 0.2 {
            character.facing_direction = Direction::Right;
            event_queue.push(PlayerSingleMoveEvent);
        }

        let vertical_value = input_handler
            .axis_value("vertical")
            .unwrap_or(0.);

        if vertical_value < -0.2 {
            character.facing_direction = Direction::Down;
            event_queue.push(PlayerSingleMoveEvent);
        } else if vertical_value > 0.2 {
            character.facing_direction = Direction::Up;
            event_queue.push(PlayerSingleMoveEvent);
        }
    }
}
