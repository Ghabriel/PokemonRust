use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn spore_causes_sleep() {
    let mut backend = battle! {
        "Paras" 27 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    let events = backend.process_turn("Spore", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition { target: 1, condition: StatusCondition::Sleep { .. } });
}

#[test]
fn spore_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Paras" 27 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    backend.process_turn("Spore", "Harden");
    let events = backend.process_turn("Spore", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
