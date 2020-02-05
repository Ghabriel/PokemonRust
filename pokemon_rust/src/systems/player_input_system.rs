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
    entities::character::{
        AllowedMovements,
        Character,
        CharacterMovement,
        MovementType,
        PlayerEntity,
    },
    events::{CharacterSingleMoveEvent, EventQueue, MapInteractionEvent},
    map::MapHandler,
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
        ReadStorage<'a, CharacterMovement>,
        ReadStorage<'a, AllowedMovements>,
        Write<'a, EventQueue>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadExpect<'a, PlayerEntity>,
        ReadExpect<'a, MapHandler>,
    );

    fn run(&mut self, (
        mut characters,
        movements,
        allowed_movements,
        mut event_queue,
        input_event_channel,
        input_handler,
        player_entity,
        map,
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

        let allowed_movements: &AllowedMovements = allowed_movements.get(player_entity.0).unwrap();
        let cancel_is_down = input_handler.action_is_down("cancel").unwrap_or(false);

        if allowed_movements.can_perform(&MovementType::Run) && cancel_is_down {
            character.action = MovementType::Run;
        } else {
            character.action = MovementType::Walk;
        }

        let character_id = map.get_character_id_by_entity(&player_entity.0);

        let horizontal_value = input_handler
            .axis_value("horizontal")
            .unwrap_or(0.);

        if horizontal_value < -0.2 {
            character.facing_direction = Direction::Left;
            event_queue.push(CharacterSingleMoveEvent::new(character_id));
        } else if horizontal_value > 0.2 {
            character.facing_direction = Direction::Right;
            event_queue.push(CharacterSingleMoveEvent::new(character_id));
        }

        let vertical_value = input_handler
            .axis_value("vertical")
            .unwrap_or(0.);

        if vertical_value < -0.2 {
            character.facing_direction = Direction::Down;
            event_queue.push(CharacterSingleMoveEvent::new(character_id));
        } else if vertical_value > 0.2 {
            character.facing_direction = Direction::Up;
            event_queue.push(CharacterSingleMoveEvent::new(character_id));
        }
    }
}
