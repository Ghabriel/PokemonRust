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
}

#[derive(Debug, Default)]
pub struct StandardBattleRng;

impl BattleRng for StandardBattleRng {
    fn get_damage_modifier(&mut self) -> f32 {
        Uniform::new(85., 100.).sample(&mut thread_rng()) / 100.
    }

    fn shuffle_moves<'a>(&mut self, moves: &mut Vec<UsedMove<'a>>) {
        moves.shuffle(&mut thread_rng());
    }
}
