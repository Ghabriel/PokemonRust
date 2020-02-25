use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};

use std::fmt::Debug;

use super::backend::UsedMove;

pub trait BattleRng: Debug {
    /// Returns a value in the range [0.85, 1].
    fn get_damage_modifier(&mut self) -> f32;

    /// Shuffles a list of moves. This ensures random move order if both the
    /// priority and speed are equal.
    fn shuffle_moves<'a>(&mut self, moves: &mut Vec<UsedMove<'a>>);

    /// Picks a number r in the range [1, 100] and returns r <= chance.
    fn check_miss(&mut self, chance: usize) -> bool;
}

#[derive(Debug, Default)]
pub struct StandardBattleRng;

impl StandardBattleRng {
    fn rand(&mut self, lowest: isize, highest: isize) -> isize {
        Uniform::new(lowest, highest + 1)
            .sample(&mut thread_rng())
    }
}

impl BattleRng for StandardBattleRng {
    fn get_damage_modifier(&mut self) -> f32 {
        self.rand(85, 100) as f32 / 100.
    }

    fn shuffle_moves<'a>(&mut self, moves: &mut Vec<UsedMove<'a>>) {
        moves.shuffle(&mut thread_rng());
    }

    fn check_miss(&mut self, chance: usize) -> bool {
        self.rand(1, 100) <= chance as isize
    }
}
