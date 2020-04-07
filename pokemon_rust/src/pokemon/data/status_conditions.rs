use crate::{
    battle::backend::{BattleBackend, DamageCause, TypeEffectiveness},
    pokemon::{
        movement::{
            ModifiedUsageAttempt,
            Move,
            MoveCategory,
        },
        SimpleStatusCondition,
        Stat,
    },
};

use std::fmt::{Debug, Error, Formatter};

#[derive(Clone)]
pub struct StatusConditionEffect {
    /// Called when backend.get_stat() is called, receiving the value that it
    /// is about to return.
    pub on_stat_calculation: Option<fn(
        backend: &BattleBackend,
        target: usize,
        stat: Stat,
        value: usize,
    ) -> usize>,

    /// Called when the target tries to use a move.
    pub on_try_use_move: Option<fn(
        backend: &mut BattleBackend,
        target: usize,
        mov: &Move,
    ) -> ModifiedUsageAttempt>,

    /// Called when a move is used and its damage is about to be dealt.
    pub on_try_deal_damage: Option<fn(
        backend: &BattleBackend,
        user: usize,
        target: usize,
        mov: &Move,
        damage_dealt: usize,
    ) -> usize>,

    /// Called when the turn ends.
    pub on_turn_end: Option<fn(backend: &mut BattleBackend, target: usize)>,
}

impl Debug for StatusConditionEffect {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str("StatusConditionEffect")
    }
}

pub fn get_status_condition_effect(
    status_condition: SimpleStatusCondition,
) -> StatusConditionEffect {
    match status_condition {
        SimpleStatusCondition::Burn => StatusConditionEffect {
            on_stat_calculation: None,
            on_try_use_move: None,
            on_try_deal_damage: Some(|_backend, _user, _target, mov, damage_dealt| {
                if mov.category == MoveCategory::Physical {
                    damage_dealt / 2
                } else {
                    damage_dealt
                }
            }),
            on_turn_end: Some(|backend, target| {
                let max_hp = backend.get_stat(target, Stat::HP) as f32;
                let damage = (max_hp / 16.).ceil() as usize;

                backend.inflict_calculated_damage(
                    target,
                    damage,
                    TypeEffectiveness::Normal,
                    false,
                    None,
                    false,
                    DamageCause::Burn,
                );
            }),
        },
        SimpleStatusCondition::Poison => StatusConditionEffect {
            on_stat_calculation: None,
            on_try_use_move: None,
            on_try_deal_damage: None,
            on_turn_end: Some(|backend, target| {
                let max_hp = backend.get_stat(target, Stat::HP) as f32;
                let damage = (max_hp / 8.).ceil() as usize;

                backend.inflict_calculated_damage(
                    target,
                    damage,
                    TypeEffectiveness::Normal,
                    false,
                    None,
                    false,
                    DamageCause::Poison,
                );
            }),
        },
        SimpleStatusCondition::Paralysis => StatusConditionEffect {
            on_stat_calculation: Some(|_backend, _target, stat, value| {
                if stat == Stat::Speed {
                    value / 2
                } else {
                    value
                }
            }),
            on_try_use_move: Some(|backend, _target, _mov| {
                if backend.check_paralysis_move_prevention() {
                    ModifiedUsageAttempt::Fail
                } else {
                    ModifiedUsageAttempt::Continue
                }
            }),
            on_try_deal_damage: None,
            on_turn_end: None,
        },
        _ => todo!(),
    }
}
