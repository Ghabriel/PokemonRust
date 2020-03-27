use crate::battle::backend::{
    rng::BattleRng,
    BattleBackend,
    BattleEvent,
    FrontendEvent,
    FrontendEventKind,
    Team,
    UsedMove,
};

// Must come first
#[macro_use]
mod macros;

mod core;
mod moves;

pub mod prelude {
    // Modules required by the test macros
    pub use crate::{
        battle::{
            backend::{
                event::{ChangeTurn, Damage, Faint, InitialSwitchIn, Miss, StatChange},
                BattleBackend,
            },
            types::{Battle, BattleCharacterTeam, BattleType, Party},
        },
        pokemon::{
            generator::PokemonBuilder,
            get_all_moves,
            get_all_pokemon_species,
            Nature,
            Stat,
        },
    };

    use crate::{
        battle::tests::TestRng,
        overworld::entities::character::CharacterId,
        pokemon::Pokemon,
    };

    pub fn create_simple_wild_battle(p1: Pokemon, p2: Pokemon) -> BattleBackend<TestRng> {
        BattleBackend::new(
            Battle::new(
                BattleType::Single,
                BattleCharacterTeam {
                    active_pokemon: None,
                    party: Party {
                        pokemon: vec![p1].into(),
                    },
                    character_id: Some(CharacterId(1)),
                },
                BattleCharacterTeam {
                    active_pokemon: None,
                    party: Party {
                        pokemon: vec![p2].into(),
                    },
                    character_id: None,
                },
            ),
            TestRng::default(),
        )
    }

    pub fn create_simple_trainer_battle(p1: Pokemon, p2: Pokemon) -> BattleBackend<TestRng> {
        BattleBackend::new(
            Battle::new(
                BattleType::Single,
                BattleCharacterTeam {
                    active_pokemon: None,
                    party: Party {
                        pokemon: vec![p1].into(),
                    },
                    character_id: Some(CharacterId(1)),
                },
                BattleCharacterTeam {
                    active_pokemon: None,
                    party: Party {
                        pokemon: vec![p2].into(),
                    },
                    character_id: Some(CharacterId(2)),
                },
            ),
            TestRng::default(),
        )
    }
}

trait TestMethods {
    fn move_p1(&mut self, index: usize);
    fn move_p2(&mut self, index: usize);
    fn process_turn(&mut self, p1_move: &str, p2_move: &str) -> Vec<BattleEvent>;
}

impl<Rng: BattleRng + Clone + 'static> TestMethods for BattleBackend<Rng> {
    fn move_p1(&mut self, index: usize) {
        self.push_frontend_event(FrontendEvent {
            team: Team::P1,
            event: FrontendEventKind::UseMove(index),
        });
    }

    fn move_p2(&mut self, index: usize) {
        self.push_frontend_event(FrontendEvent {
            team: Team::P2,
            event: FrontendEventKind::UseMove(index),
        });
    }

    fn process_turn(&mut self, p1_move: &str, p2_move: &str) -> Vec<BattleEvent> {
        let p1_index = self.p1.active_pokemon.unwrap();
        let p2_index = self.p2.active_pokemon.unwrap();

        let p1_move_index = self.pokemon_repository[&p1_index]
            .moves
            .iter()
            .enumerate()
            .filter_map(|(i, mov)| match mov {
                Some(mov) => Some((i, mov)),
                None => None,
            })
            .find(|(_, mov)| mov.as_str() == p1_move)
            .map(|(i, _)| i)
            .unwrap_or_else(|| panic!("Move \"{}\" not found for player 1", p1_move));

        let p2_move_index = self.pokemon_repository[&p2_index]
            .moves
            .iter()
            .enumerate()
            .filter_map(|(i, mov)| match mov {
                Some(mov) => Some((i, mov)),
                None => None,
            })
            .find(|(_, mov)| mov.as_str() == p2_move)
            .map(|(i, _)| i)
            .unwrap_or_else(|| panic!("Move \"{}\" not found for player 2", p2_move));

        self.move_p1(p1_move_index);
        self.move_p2(p2_move_index);

        self.tick().collect()
    }
}

#[derive(Clone, Debug, Default)]
pub struct TestRng {
    miss_counter: usize,
    last_miss_check_chance: Option<usize>,
    last_secondary_effect_check_chance: Option<usize>,
    uniform_multi_hit_value: Option<usize>,
    custom_multi_hit_value: Option<isize>,
}

impl TestRng {
    pub fn force_miss(&mut self, times: usize) {
        self.miss_counter = times;
    }

    pub fn get_last_miss_check_chance(&self) -> Option<usize> {
        self.last_miss_check_chance
    }

    pub fn force_uniform_multi_hit_value(&mut self, value: usize) {
        self.uniform_multi_hit_value = Some(value);
    }

    pub fn force_custom_multi_hit_value(&mut self, value: isize) {
        self.custom_multi_hit_value = Some(value);
    }
}

impl BattleRng for TestRng {
    fn get_damage_modifier(&mut self) -> f32 {
        1.
    }

    fn shuffle_moves<'a>(&mut self, _moves: &mut Vec<UsedMove<'a>>) {}

    fn check_miss(&mut self, chance: usize) -> bool {
        self.last_miss_check_chance = Some(chance);

        if self.miss_counter > 0 {
            self.miss_counter -= 1;
            true
        } else {
            false
        }
    }

    fn check_secondary_effect(&mut self, chance: usize) -> bool {
        self.last_secondary_effect_check_chance = Some(chance);

        chance == 100
    }

    fn check_uniform_multi_hit(&mut self, lowest: usize, highest: usize) -> usize {
        match self.uniform_multi_hit_value {
            Some(value) => value,
            None => highest,
        }
    }

    fn check_custom_multi_hit(&mut self, lowest: isize, highest: isize) -> isize {
        match self.custom_multi_hit_value {
            Some(value) => value,
            None => highest,
        }
    }
}
