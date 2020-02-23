//! A system responsible for processing Pokémon battles.
use amethyst::ecs::System;

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
pub struct BattleSystem;

impl<'a> System<'a> for BattleSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {
        // TODO
    }
}
