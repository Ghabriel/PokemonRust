use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};

use std::fmt::Debug;

use super::UsedMove;

pub trait BattleRng: Debug {
    /// Returns a value in the range [0.85, 1].
    fn get_damage_modifier(&mut self) -> f32;

    /// Shuffles a list of moves. This ensures random move order if both the
    /// priority and speed are equal.
    fn shuffle_moves<'a>(&mut self, moves: &mut Vec<UsedMove<'a>>);

    /// Picks a number r in the range [1, 100] and returns r <= chance.
    fn check_miss(&mut self, chance: usize) -> bool;

    /// Picks a number r in the range [1, 100] and returns r <= chance.
    fn check_secondary_effect(&mut self, chance: usize) -> bool;

    /// Returns a number r in the range [lowest, highest] used to calculate the
    /// number of hits of a uniform multi-hit move.
    fn check_uniform_multi_hit(&mut self, lowest: usize, highest: usize) -> usize;

    /// Returns a number r in the range [lowest, highest] used to calculate the
    /// number of hits of a custom multi-hit move.
    fn check_custom_multi_hit(&mut self, lowest: isize, highest: isize) -> isize;
}

#[derive(Clone, Debug, Default)]
pub struct StandardBattleRng;

impl StandardBattleRng {
    fn rand(&mut self, lowest: isize, highest: isize) -> isize {
        Uniform::new(lowest, highest + 1).sample(&mut thread_rng())
    }

    fn rand_unsigned(&mut self, lowest: usize, highest: usize) -> usize {
        Uniform::new(lowest, highest + 1).sample(&mut thread_rng())
    }

    fn roll(&mut self, chance: usize) -> bool {
        self.rand(1, 100) <= chance as isize
    }
}

impl BattleRng for StandardBattleRng {
    fn get_damage_modifier(&mut self) -> f32 {
        self.rand(85, 100) as f32 / 100.
    }

    fn shuffle_moves<'a>(&mut self, moves: &mut Vec<UsedMove<'a>>) {
        moves.shuffle(&mut thread_rng());
    }

    fn check_miss(&mut self, accuracy: usize) -> bool {
        self.roll(100 - accuracy)
    }

    fn check_secondary_effect(&mut self, chance: usize) -> bool {
        self.roll(chance)
    }

    fn check_uniform_multi_hit(&mut self, lowest: usize, highest: usize) -> usize {
        self.rand_unsigned(lowest, highest)
    }

    fn check_custom_multi_hit(&mut self, lowest: isize, highest: isize) -> isize {
        self.rand(lowest, highest)
    }
}
