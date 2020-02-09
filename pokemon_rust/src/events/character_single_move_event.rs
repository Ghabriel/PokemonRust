use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    constants::TILE_SIZE,
    entities::character::{AllowedMovements, Character, CharacterMovement},
    map::{MapHandler, PlayerCoordinates, TileData},
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct CharacterSingleMoveEvent {
    character_id: usize,
}

impl CharacterSingleMoveEvent {
    pub fn new(character_id: usize) -> CharacterSingleMoveEvent {
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

        let movement = CharacterMovement {
            estimated_time: f32::from(TILE_SIZE) / velocity,
            velocity,
            movement_type: character.action.clone(),
            step_kind: character.next_step.clone(),
            started: false,
            from: TileData {
                position: character_position.clone(),
                // TODO: use the NPC's natural map
                map_id: map_handler.get_current_map_id(),
            },
            // TODO: use the NPC's natural map
            to: map_handler.get_forward_tile(&character.facing_direction, &character_position),
        };

        world.write_storage::<CharacterMovement>()
            .insert(*entity, movement)
            .expect("Failed to attach CharacterMovement");
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_character_by_id(self.character_id);

        let has_pending_interaction = world
            .read_storage::<Character>()
            .get(*entity)
            .unwrap()
            .pending_interaction;

        let is_moving = world.read_storage::<CharacterMovement>().contains(*entity);

        !has_pending_interaction && !is_moving
    }
}
