use amethyst::{
    core::{Time, Transform},
    ecs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage},
};

use crate::entities::player::{Direction, Player, PlayerAction, SimulatedPlayer};

use std::collections::HashMap;

struct MovementTimingData {
    /// Stores how much time it will take for the player to reach the next tile.
    estimated_time: f32,
}

#[derive(Default)]
pub struct PlayerMovementSystem {
    timing_data: HashMap<Entity, MovementTimingData>,
}

const TILE_SIZE: f32 = 32.;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        ReadStorage<'a, SimulatedPlayer>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut players,
        simulated_players,
        mut transforms,
        entities,
        time,
    ): Self::SystemData) {
        let mut static_players = Vec::new();

        for (entity, player, transform) in (&entities, &players, &mut transforms).join() {
            let velocity = match player.action {
                PlayerAction::Idle => unreachable!(),
                PlayerAction::Walk => 160.,
                PlayerAction::Run => 256.,
            };

            let timing_data = self.timing_data.get_mut(&entity);

            match timing_data {
                Some(timing_data) => {
                    let delta_seconds = time.delta_seconds();

                    if timing_data.estimated_time <= delta_seconds {
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

                    let estimated_time = TILE_SIZE / velocity;
                    self.timing_data.insert(entity, MovementTimingData { estimated_time });
                },
            }

            let (offset_x, offset_y) = match player.facing_direction {
                Direction::Up => (0., 1.),
                Direction::Down => (0., -1.),
                Direction::Left => (-1., 0.),
                Direction::Right => (1., 0.),
            };

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
