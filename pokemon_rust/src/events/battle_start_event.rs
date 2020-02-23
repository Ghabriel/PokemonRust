//! Initiates a Pokémon battle.

use amethyst::ecs::{World, WorldExt};

use crate::{
    battle::types::{Battle, BattleCharacterTeam, BattleType, Party},
    entities::{
        character::{CharacterId, PlayerEntity},
        pokemon::{
            generator::generate_pokemon,
            get_all_moves,
            get_all_pokemon_species,
        },
    },
    map::MapHandler,
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
        let player_entity = world.read_resource::<PlayerEntity>().0;
        let player_id = world
            .read_resource::<MapHandler>()
            .get_character_id_by_entity(player_entity);

        let pokedex = get_all_pokemon_species();
        let movedex = get_all_moves();

        let battle_type = self.battle_type.clone();

        let party = {
            let rattata = generate_pokemon(
                &pokedex.get_species("Rattata").unwrap(),
                &movedex,
                3,
            );

            Party {
                pokemon: vec![rattata],
            }
        };

        // TODO: do this somewhere to persist the player's Party
        // world
        //     .write_storage::<Party>()
        //     .insert(player_entity, party)
        //     .expect("Failed to attach Party");

        let p1 = BattleCharacterTeam {
            active_pokemon: None,
            party,
            character_id: Some(player_id),
        };

        let p2 = {
            let pidgey = generate_pokemon(
                &pokedex.get_species("Pidgey").unwrap(),
                &movedex,
                3,
            );

            BattleCharacterTeam {
                active_pokemon: None,
                party: Party { pokemon: vec![pidgey] },
                character_id: match self.opponent {
                    BattleOpponent::Trainer(character_id) => Some(character_id),
                    BattleOpponent::WildPokemon => None,
                }
            }
        };

        world.insert(Battle { battle_type, p1, p2 });
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, world: &mut World) -> bool {
        !world.has_value::<Battle>()
    }
}
