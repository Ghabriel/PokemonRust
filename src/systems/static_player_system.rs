use amethyst::{
    ecs::{
        Entities,
        Join,
        ReadStorage,
        System,
        WriteStorage,
    },
};

use crate::{
    entities::player::{Player, SimulatedPlayer, StaticPlayer},
};

pub struct StaticPlayerSystem;

impl<'a> System<'a> for StaticPlayerSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        ReadStorage<'a, SimulatedPlayer>,
        ReadStorage<'a, StaticPlayer>,
        Entities<'a>,
    );

    fn run(&mut self, (
        mut players,
        simulated_players,
        static_players,
        entities,
    ): Self::SystemData) {
        for (
            entity,
            simulated_player,
            _,
        ) in (&entities, &simulated_players, &static_players).join() {
            // Don't mutably borrow the player right away to avoid triggering
            // an unnecessary animation refresh (see PlayerAnimationSystem).
            let needs_mutation = {
                let player = players
                    .get(entity)
                    .expect("Failed to retrieve Player");

                simulated_player.0 != *player
            };

            if needs_mutation {
                let player = players
                    .get_mut(entity)
                    .expect("Failed to retrieve Player");

                *player = simulated_player.0.clone();
            }
        }
    }
}
