use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::{Direction, get_character_sprite_index_from_direction},
    entities::character::{Character, PlayerEntity},
    map::{MapHandler, PlayerCoordinates},
};

use super::{BoxedGameEvent, GameEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct CharacterRotateEvent {
    character_id: usize,
    direction_type: DirectionType,
}

#[derive(Clone)]
enum DirectionType {
    Fixed(Direction),
    TowardsPlayer,
}

impl DirectionType {
    fn get_direction(&self, world: &World, character_id: usize) -> Option<Direction> {
        match self {
            DirectionType::Fixed(direction) => Some(direction.clone()),
            DirectionType::TowardsPlayer => {
                let map_handler = world.read_resource::<MapHandler>();
                let npc_entity = map_handler.get_character_by_id(character_id);

                let npc_position = world.read_storage::<Transform>()
                    .get(*npc_entity)
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
    pub fn new(character_id: usize, direction: Direction) -> CharacterRotateEvent {
        CharacterRotateEvent {
            character_id,
            direction_type: DirectionType::Fixed(direction),
        }
    }

    pub fn towards_player(character_id: usize) -> CharacterRotateEvent {
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

    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        ShouldDisableInput(false)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_character_by_id(self.character_id);

        if let Some(direction) = self.direction_type.get_direction(world, self.character_id) {
            world.write_storage::<SpriteRender>()
                .get_mut(*entity)
                .unwrap()
                .sprite_number = get_character_sprite_index_from_direction(&direction);

            world.write_storage::<Character>()
                .get_mut(*entity)
                .unwrap()
                .facing_direction = direction;
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
