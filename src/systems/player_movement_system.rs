use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
};

use crate::{
    common::get_forward_tile_position,
    constants::TILE_SIZE,
    entities::{
        map::Map,
        player::{Direction, Player, PlayerAction, SimulatedPlayer},
    },
};

use std::collections::HashMap;

struct MovementTimingData {
    /// Stores how much time it will take for the player to reach the next tile.
    estimated_time: f32,
    /// Stores where the player will be after he reaches the next tile. This
    /// is used to compensate for rounding errors.
    final_position: Vector3<f32>,
}

#[derive(Default)]
pub struct PlayerMovementSystem {
    timing_data: HashMap<Entity, MovementTimingData>,
}

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        ReadStorage<'a, SimulatedPlayer>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        ReadExpect<'a, Map>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut players,
        simulated_players,
        mut transforms,
        entities,
        map,
        time,
    ): Self::SystemData) {
        let mut static_players = Vec::new();

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
                        transform.set_translation(timing_data.final_position);
                        self.timing_data.remove(&entity);
                        static_players.push(entity);
                        continue;
                    }

                    timing_data.estimated_time -= delta_seconds;
                },
                None => {
                    if !player.moving {
                        static_players.push(entity);
                        continue;
                    }

                    let final_position = get_forward_tile_position(&player, &transform);

                    if map.is_tile_blocked(&final_position) {
                        static_players.push(entity);
                        continue;
                    }

                    let estimated_time = (TILE_SIZE as f32) / velocity;

                    self.timing_data.insert(entity, MovementTimingData {
                        estimated_time,
                        final_position,
                    });
                },
            }

            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }

        for entity in static_players {
            let simulated_player = simulated_players
                .get(entity)
                .expect("Failed to retrieve SimulatedPlayer");

            let player = players
                .get(entity)
                .expect("Failed to retrieve Player");

            if simulated_player.0 != *player {
                let player = players
                    .get_mut(entity)
                    .expect("Failed to retrieve Player");

                *player = simulated_player.0.clone();
            }
        }
    }
}
