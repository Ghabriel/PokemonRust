//! Initiates a Pokémon battle.

use amethyst::ecs::{world::Builder, World, WorldExt};

use crate::{
    entities::{
        battle::{Battle, BattleCharacterTeam, BattleType},
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
        let pokedex = get_all_pokemon_species();
        let movedex = get_all_moves();

        let battle_type = self.battle_type.clone();

        let p1 = BattleCharacterTeam::Trainer {
            character_id: get_player_character_id(world),
        };

        let p2 = match self.opponent {
            BattleOpponent::Trainer(character_id) => {
                BattleCharacterTeam::Trainer { character_id }
            },
            BattleOpponent::WildPokemon => {
                let pokemon = generate_pokemon(
                    &pokedex.get_species("Pidgey").unwrap(),
                    &movedex,
                    3,
                );

                BattleCharacterTeam::WildPokemon { pokemon }
            },
        };

        world.insert(pokedex);
        world.insert(movedex);

        world
            .create_entity()
            .with(Battle { battle_type, p1, p2 })
            .build();
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}

fn get_player_character_id(world: &World) -> CharacterId {
    let player_entity = world.read_resource::<PlayerEntity>().0;

    world
        .read_resource::<MapHandler>()
        .get_character_id_by_entity(player_entity)
}
