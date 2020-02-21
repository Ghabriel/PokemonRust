//! Initiates a Pokémon battle.

use amethyst::ecs::{World, WorldExt};

use crate::{
    entities::{
        battle::BattleType,
        character::CharacterId,
        pokemon::{
            generator::generate_pokemon,
            get_all_moves,
            get_all_pokemon_species,
            movement::MoveDex,
            PokeDex,
        },
    },
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct BattleStartEvent {
    battle_type: BattleType,
    opponent: BattleOpponent,
}

#[derive(Clone)]
enum BattleOpponent {
    Trainer(CharacterId),
    WildPokemon,
}

impl BattleStartEvent {
    pub fn against_trainer(
        battle_type: BattleType,
        character_id: CharacterId,
    ) -> BattleStartEvent {
        BattleStartEvent {
            battle_type,
            opponent: BattleOpponent::Trainer(character_id),
        }
    }

    pub fn wild(battle_type: BattleType) -> BattleStartEvent {
        BattleStartEvent {
            battle_type,
            opponent: BattleOpponent::WildPokemon,
        }
    }
}

impl GameEvent for BattleStartEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
            requires_battle_state: true,
        }
    }

    fn start(&mut self, world: &mut World) {
        let pokedex = get_all_pokemon_species();
        let movedex = get_all_moves();
        let pidgey = generate_pokemon(
            &pokedex.get_species("Pidgey").unwrap(),
            &movedex,
            3,
        );

        println!("{:#?}", pidgey);
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
