//! Rotates a character towards a direction.

use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    common::Direction,
    entities::{
        AnimationTable,
        character::{Character, CharacterAnimation, CharacterId, PlayerEntity},
    },
    map::{MapHandler, PlayerCoordinates},
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct CharacterRotateEvent {
    character_id: CharacterId,
    direction_type: DirectionType,
}

#[derive(Clone)]
enum DirectionType {
    Fixed(Direction),
    TowardsPlayer,
}

impl DirectionType {
    fn get_direction(&self, world: &World, character_id: CharacterId) -> Option<Direction> {
        match self {
            DirectionType::Fixed(direction) => Some(direction.clone()),
            DirectionType::TowardsPlayer => {
                let npc_entity = world
                    .read_resource::<MapHandler>()
                    .get_character_by_id(character_id);

                let npc_position = world.read_storage::<Transform>()
                    .get(npc_entity)
                    .map(PlayerCoordinates::from_transform)
                    .unwrap();

                let player_entity = world.read_resource::<PlayerEntity>();

                let player_position = world.read_storage::<Transform>()
                        .get(player_entity.0)
                        .map(PlayerCoordinates::from_transform)
                        .unwrap();

                npc_position.get_direction_to(&player_position)
            }
        }
    }
}

impl CharacterRotateEvent {
    /// Instantiates a CharacterRotateEvent that rotates a character towards
    /// a given direction.
    pub fn new(character_id: CharacterId, direction: Direction) -> CharacterRotateEvent {
        CharacterRotateEvent {
            character_id,
            direction_type: DirectionType::Fixed(direction),
        }
    }

    /// Instantiates a CharacterRotateEvent that rotates a character towards
    /// the human player.
    pub fn towards_player(character_id: CharacterId) -> CharacterRotateEvent {
        CharacterRotateEvent {
            character_id,
            direction_type: DirectionType::TowardsPlayer,
        }
    }
}

impl GameEvent for CharacterRotateEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
        }
    }

    fn start(&mut self, _world: &mut World) { }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let entity = world
            .read_resource::<MapHandler>()
            .get_character_by_id(self.character_id);

        if let Some(direction) = self.direction_type.get_direction(world, self.character_id) {
            world.write_storage::<AnimationTable<CharacterAnimation>>()
                .get_mut(entity)
                .unwrap()
                .change_animation(CharacterAnimation::Idle(
                    direction.clone(),
                ));

            world.write_storage::<Character>()
                .get_mut(entity)
                .unwrap()
                .facing_direction = direction;
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
