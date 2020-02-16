//! Moves a character one tile forward. See the
//! [CharacterMovementSystem](../systems/character_movement_system/struct.CharacterMovementSystem.html)
//! for details on which situations this event can "hang".

use amethyst::ecs::{World, WorldExt};

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
    map::{MapHandler, TileDataBuilder},
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
        let entity = world
            .read_resource::<MapHandler>()
            .get_character_by_id(self.character_id);

        let character = world
            .read_storage::<Character>()
            .get(entity)
            .unwrap()
            .clone();

        let velocity = world.read_storage::<AllowedMovements>()
            .get(entity)
            .unwrap()
            .get_movement_data(&character.action)
            .unwrap()
            .velocity;

        let initial_tile_data = TileDataBuilder::default()
            .with_entity(entity)
            .with_character_id(self.character_id)
            .build(world);

        let final_tile_data = world
            .read_resource::<MapHandler>()
            .get_forward_tile(&character.facing_direction, &initial_tile_data);

        let movement = CharacterMovement {
            estimated_time: f32::from(TILE_SIZE) / velocity,
            velocity,
            movement_type: character.action,
            step_kind: character.next_step,
            started: false,
            from: initial_tile_data,
            to: final_tile_data,
        };

        world.write_storage::<CharacterMovement>()
            .insert(entity, movement)
            .expect("Failed to attach CharacterMovement");
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let entity = world
            .read_resource::<MapHandler>()
            .get_character_by_id(self.character_id);

        let has_pending_interaction = world.has_value::<PendingInteraction>();

        let has_text_box = !world
            .read_storage::<TextBox>()
            .is_empty();

        let is_moving = world.read_storage::<CharacterMovement>().contains(entity);

        !has_pending_interaction && !has_text_box && !is_moving
    }
}
