use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    constants::TILE_SIZE,
    entities::{
        character::{
            AllowedMovements,
            Character,
            CharacterId,
            CharacterMovement,
            PendingInteraction,
        },
        text_box::TextBox,
    },
    map::{MapHandler, PlayerCoordinates, TileData},
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct CharacterSingleMoveEvent {
    character_id: CharacterId,
}

impl CharacterSingleMoveEvent {
    pub fn new(character_id: CharacterId) -> CharacterSingleMoveEvent {
        CharacterSingleMoveEvent {
            character_id,
        }
    }
}

impl GameEvent for CharacterSingleMoveEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
        }
    }

    fn start(&mut self, world: &mut World) {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_character_by_id(self.character_id);

        let characters = world.read_storage::<Character>();
        let character = characters.get(*entity).unwrap();

        let character_position = world.read_storage::<Transform>()
            .get(*entity)
            .map(PlayerCoordinates::from_transform)
            .unwrap();

        let velocity = world.read_storage::<AllowedMovements>()
            .get(*entity)
            .unwrap()
            .get_movement_data(&character.action)
            .unwrap()
            .velocity;

        let tile_data = TileData {
            position: character_position.clone(),
            map_id: map_handler.get_character_current_map(self.character_id).clone(),
        };

        let movement = CharacterMovement {
            estimated_time: f32::from(TILE_SIZE) / velocity,
            velocity,
            movement_type: character.action.clone(),
            step_kind: character.next_step.clone(),
            started: false,
            to: map_handler.get_forward_tile(&character.facing_direction, &tile_data),
            from: tile_data,
        };

        world.write_storage::<CharacterMovement>()
            .insert(*entity, movement)
            .expect("Failed to attach CharacterMovement");
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_character_by_id(self.character_id);

        let has_pending_interaction = world.has_value::<PendingInteraction>();

        let has_text_box = !world
            .read_storage::<TextBox>()
            .is_empty();

        let is_moving = world.read_storage::<CharacterMovement>().contains(*entity);

        !has_pending_interaction && !has_text_box && !is_moving
    }
}
