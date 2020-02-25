use crate::battle::{
    backend::{BattleBackend, BattleEvent, FrontendEvent, FrontendEventKind, Team, UsedMove},
    rng::BattleRng,
};

// Must come first
#[macro_use]
mod macros;

mod core;

trait TestMethods {
    fn move_p1(&mut self, index: usize);
    fn move_p2(&mut self, index: usize);
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
}

#[derive(Debug)]
struct TestRng;

impl BattleRng for TestRng {
    fn get_damage_modifier(&mut self) -> f32 {
        1.
    }

    fn shuffle_moves<'a>(&mut self, _moves: &mut Vec<UsedMove<'a>>) {}
}

fn process_turn<'a, Rng: BattleRng>(
    backend: &'a mut BattleBackend<Rng>,
    p1_move: &str,
    p2_move: &str,
) -> impl Iterator<Item = BattleEvent> + 'a {
    let p1_index = backend.p1.active_pokemon.unwrap();
    let p2_index = backend.p2.active_pokemon.unwrap();

    let p1_move_index = backend
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
        .unwrap();

    let p2_move_index = backend
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
        .unwrap();

    backend.move_p1(p1_move_index);
    backend.move_p2(p2_move_index);
    backend.tick()
}
