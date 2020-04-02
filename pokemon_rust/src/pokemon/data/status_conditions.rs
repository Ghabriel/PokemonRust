use crate::{
    battle::backend::{BattleBackend, TypeEffectiveness},
    pokemon::{
        movement::{
            Move,
            MoveCategory,
        },
        SimpleStatusCondition,
        Stat,
    },
};

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

pub struct StatusConditionEffect {
    on_try_deal_damage: fn(
        backend: &BattleBackend,
        user: usize,
        target: usize,
        mov: &Move,
        damage_dealt: usize,
    ) -> usize,

    on_turn_end: fn(backend: &mut BattleBackend, target: usize),
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
            on_try_deal_damage: |backend, _user, _target, mov, damage_dealt| {
                if mov.category == MoveCategory::Physical {
                    damage_dealt / 2
                } else {
                    damage_dealt
                }
            },
            on_turn_end: |backend, target| {
                let max_hp = backend.get_stat(target, Stat::HP) as f32;
                let damage = (max_hp / 16.).ceil() as usize;

                backend.inflict_calculated_damage(
                    target,
                    damage,
                    TypeEffectiveness::Normal,
                    false,
                    None,
                    false,
                );
            }
        },
        _ => todo!(),
    }
}
