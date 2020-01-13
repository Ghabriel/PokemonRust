use amethyst::{
    core::{Time, Transform},
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    shrev::EventChannel,
};

use crate::{
    common::Direction,
    constants::TILE_SIZE,
    entities::{
        map::{MapHandler, MapScriptKind, ScriptEvent, TileData},
        player::{Player, PlayerAction, StaticPlayer},
    },
};

use std::collections::HashMap;

struct MovementTimingData {
    /// Stores how much time it will take for the player to reach the next tile.
    estimated_time: f32,
    /// Stores where the player will be after he reaches the next tile. This
    /// is used to compensate for rounding errors.
    final_tile_data: TileData,
}

#[derive(Default)]
pub struct PlayerMovementSystem {
    timing_data: HashMap<Entity, MovementTimingData>,
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, StaticPlayer>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        ReadExpect<'a, MapHandler>,
        Write<'a, EventChannel<ScriptEvent>>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        players,
        mut static_players,
        mut transforms,
        entities,
        map,
        mut script_event_channel,
        time,
    ): Self::SystemData) {
        for (entity, player, transform) in (&entities, &players, &mut transforms).join() {
            let velocity = match player.action {
                PlayerAction::Idle => unreachable!(),
                PlayerAction::Walk => 160.,
                PlayerAction::Run => 256.,
            };

            let (offset_x, offset_y) = match player.facing_direction {
                Direction::Up => (0., 1.),
                Direction::Down => (0., -1.),
                Direction::Left => (-1., 0.),
                Direction::Right => (1., 0.),
            };

            let timing_data = self.timing_data.get_mut(&entity);

            match timing_data {
                Some(timing_data) => {
                    let delta_seconds = time.delta_seconds();

                    if timing_data.estimated_time <= delta_seconds {
                        transform.set_translation(timing_data.final_tile_data.position);

                        map.get_map_scripts(&timing_data.final_tile_data, MapScriptKind::OnTileChange)
                            .for_each(|event| {
                                script_event_channel.single_write(event);
                            });

                        self.timing_data.remove(&entity);
                        static_players
                            .insert(entity, StaticPlayer)
                            .expect("Failed to attach StaticPlayer");

                        continue;
                    }

                    timing_data.estimated_time -= delta_seconds;
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

                    self.timing_data.insert(entity, MovementTimingData {
                        estimated_time,
                        final_tile_data,
                    });
                },
            }

            static_players.remove(entity);
            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }
    }
}
