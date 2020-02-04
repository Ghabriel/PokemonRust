use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    config::GameConfig,
    constants::TILE_SIZE,
    entities::{
        character::{Character, CharacterMovement, MovementType},
        player::PlayerEntity,
    },
    map::{MapHandler, PlayerCoordinates, TileData},
};

use super::{GameEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct CharacterSingleMoveEvent {
    npc_id: usize,
}

impl CharacterSingleMoveEvent {
    pub fn new(npc_id: usize) -> CharacterSingleMoveEvent {
        CharacterSingleMoveEvent {
            npc_id,
        }
    }
}

impl GameEvent for CharacterSingleMoveEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_npc_by_id(self.npc_id);

        let characters = world.read_storage::<Character>();
        let character = characters.get(*entity).unwrap();

        let character_position = world.read_storage::<Transform>()
            .get(*entity)
            .map(PlayerCoordinates::from_transform)
            .unwrap();

        let config = world.read_resource::<GameConfig>();

        let player_entity = world.read_resource::<PlayerEntity>().0;

        let velocity = if *entity == player_entity {
            match character.action {
                MovementType::Walk => config.player_walking_speed,
                MovementType::Run => config.player_running_speed,
            }
        } else {
            // TODO: extract velocity to constant or use GameConfig::player_walking_speed
            160.
        };

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

        ShouldDisableInput(false)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let map_handler = world.read_resource::<MapHandler>();
        let entity = map_handler.get_npc_by_id(self.npc_id);

        !world.read_storage::<CharacterMovement>()
            .contains(*entity)
    }
}
