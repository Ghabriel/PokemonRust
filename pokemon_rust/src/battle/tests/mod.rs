use crate::battle::{
    backend::{BattleBackend, BattleEvent, FrontendEvent, FrontendEventKind, Team, UsedMove},
    rng::BattleRng,
};

// Must come first
#[macro_use]
mod macros;

mod core;

pub mod prelude {
    // Modules required by the test macros
    pub use crate::{
        battle::{
            backend::BattleBackend,
            types::{Battle, BattleCharacterTeam, BattleType, Party},
        },
        entities::pokemon::{
            generator::PokemonBuilder,
            get_all_moves,
            get_all_pokemon_species,
            Nature,
        },
    };
}

mod karate_chop;
mod pound;
mod tackle;
mod tailwhip;

trait TestMethods {
    fn move_p1(&mut self, index: usize);
    fn move_p2(&mut self, index: usize);
    fn process_turn(&mut self, p1_move: &str, p2_move: &str) -> Vec<BattleEvent>;
}

impl<Rng: BattleRng> TestMethods for BattleBackend<Rng> {
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

        let p1_move_index = self
            .pokemon_repository[&p1_index]
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

        let p2_move_index = self
            .pokemon_repository[&p2_index]
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

#[derive(Debug, Default)]
struct TestRng {
    miss_counter: usize,
    last_miss_check_chance: Option<usize>,
    last_secondary_effect_check_chance: Option<usize>,
}

impl TestRng {
    pub fn force_miss(&mut self, times: usize) {
        self.miss_counter = times;
    }

    pub fn get_last_miss_check_chance(&self) -> Option<usize> {
        self.last_miss_check_chance
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
}
