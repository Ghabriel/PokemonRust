use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    config::GameConfig,
    constants::TILE_SIZE,
    entities::{
        character::Character,
        player::{Player, PlayerAction, PlayerEntity, PlayerMovement},
    },
    map::{MapHandler, PlayerCoordinates, TileData},
};

use super::{GameEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct PlayerSingleMoveEvent;

impl GameEvent for PlayerSingleMoveEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        let players = world.read_storage::<Player>();
        let player_entity = world.read_resource::<PlayerEntity>().0;

        let characters = world.read_storage::<Character>();
        let character = characters.get(player_entity).unwrap();
        let player = players.get(player_entity).unwrap();

        let player_position = world.read_storage::<Transform>()
            .get(player_entity)
            .map(PlayerCoordinates::from_transform)
            .unwrap();

        let config = world.read_resource::<GameConfig>();

        let velocity = match player.action {
            PlayerAction::Idle => unreachable!(),
            PlayerAction::Walk => config.player_walking_speed,
            PlayerAction::Run => config.player_running_speed,
        };

        let map_handler = world.read_resource::<MapHandler>();

        let movement = PlayerMovement {
            estimated_time: f32::from(TILE_SIZE) / velocity,
            velocity,
            action: player.action.clone(),
            step_kind: character.next_step.clone(),
            started: false,
            from: TileData {
                position: player_position.clone(),
                map_id: map_handler.get_current_map_id(),
            },
            to: map_handler.get_forward_tile(&character.facing_direction, &player_position),
        };

        world.write_storage::<PlayerMovement>()
            .insert(player_entity, movement)
            .expect("Failed to attach PlayerMovement");

        ShouldDisableInput(false)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let player_entity = world.read_resource::<PlayerEntity>().0;

        !world.read_storage::<PlayerMovement>()
            .contains(player_entity)
    }
}
