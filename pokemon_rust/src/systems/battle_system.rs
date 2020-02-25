//! A system responsible for processing Pokémon battles.
use amethyst::ecs::{System, WriteExpect};

use crate::battle::{
    backend::{BattleBackend, BattleEvent},
    rng::StandardBattleRng,
    types::Battle,
};

use std::collections::VecDeque;

/// A system responsible for processing Pokémon battles. Architecturally,
/// battles are split into two layers: one acts like a "frontend" and the other
/// acts like a "backend". This separation allows the battle mechanics
/// themselves to be independent of their visual representation and processing,
/// improving testability and maintainability considerably.
///
/// With that archicture in mind, this system is the "frontend". The frontend
/// is responsible for receiving events from the backend and displaying them
/// to the screen in an intuitive way. It also handles the player's input,
/// sending signals to the backend whenever an action is taken.
#[derive(Default)]
pub struct BattleSystem {
    backend: Option<BattleBackend<StandardBattleRng>>,
    event_queue: VecDeque<BattleEvent>,
}

impl BattleSystem {
    fn init_backend(
        &mut self,
        (battle,): <Self as System>::SystemData,
    ) {
        self.backend = Some(BattleBackend::new(
            (*battle).clone(),
            StandardBattleRng::default(),
        ));
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = (
        WriteExpect<'a, Battle>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        let backend = match self.backend.as_mut() {
            Some(backend) => backend,
            None => {
                self.init_backend(system_data);
                self.backend.as_mut().unwrap()
            },
        };

        while self.event_queue.is_empty() {
            self.event_queue.extend(backend.tick());
        }

        while let Some(event) = self.event_queue.pop_front() {
            println!("{:?}", event);
        }
    }
}
