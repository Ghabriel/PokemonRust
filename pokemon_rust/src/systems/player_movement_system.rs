use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};

use crate::{
    common::get_direction_offset,
    config::GameConfig,
    constants::TILE_SIZE,
    entities::{
        map::{change_tile, MapHandler, MapId, ScriptEvent, TileData},
        player::{Player, PlayerAction, StaticPlayer},
    },
};

use std::collections::HashMap;

struct MovementData {
    /// Stores how much time it will take for the player to reach the next tile.
    estimated_time: f32,
    /// Stores the map that the player was in at the start of the movement.
    /// Useful for detecting map changes.
    starting_map_id: MapId,
    /// Stores where the player will be after he reaches the next tile. This
    /// is used to compensate for rounding errors and detecting map changes.
    final_tile_data: TileData,
}

#[derive(Default)]
pub struct PlayerMovementSystem {
    movement_data: HashMap<Entity, MovementData>,
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, StaticPlayer>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        ReadExpect<'a, GameConfig>,
        ReadExpect<'a, MapHandler>,
        Write<'a, EventChannel<ScriptEvent>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        players,
        mut static_players,
        mut transforms,
        entities,
        config,
        map,
        mut script_event_channel,
        time,
    ): Self::SystemData) {
        for (entity, player, transform) in (&entities, &players, &mut transforms).join() {
            let velocity = match player.action {
                PlayerAction::Idle => unreachable!(),
                PlayerAction::Walk => config.player_walking_speed,
                PlayerAction::Run => config.player_running_speed,
            };

            let movement_data = self.movement_data.get_mut(&entity);

            match movement_data {
                Some(movement_data) => {
                    let delta_seconds = time.delta_seconds();

                    if movement_data.estimated_time <= delta_seconds {
                        transform.set_translation(Vector3::new(
                            movement_data.final_tile_data.position.0.x,
                            movement_data.final_tile_data.position.0.y,
                            0.,
                        ));

                        change_tile(
                            &movement_data.starting_map_id,
                            &movement_data.final_tile_data,
                            &map,
                            &mut script_event_channel,
                        );

                        self.movement_data.remove(&entity);
                        static_players
                            .insert(entity, StaticPlayer)
                            .expect("Failed to attach StaticPlayer");

                        continue;
                    }

                    movement_data.estimated_time -= delta_seconds;
                },
                None => {
                    if !player.moving {
                        static_players
                            .insert(entity, StaticPlayer)
                            .expect("Failed to attach StaticPlayer");
                        continue;
                    }

                    let final_tile_data = map.get_forward_tile(&player, &transform);

                    if map.is_tile_blocked(&final_tile_data) {
                        static_players
                            .insert(entity, StaticPlayer)
                            .expect("Failed to attach StaticPlayer");
                        continue;
                    }

                    let estimated_time = (TILE_SIZE as f32) / velocity;

                    self.movement_data.insert(entity, MovementData {
                        estimated_time,
                        starting_map_id: map.get_current_map_id(),
                        final_tile_data,
                    });
                },
            }

            static_players.remove(entity);
            let (offset_x, offset_y) = get_direction_offset::<f32>(&player.facing_direction);
            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }
    }
}
