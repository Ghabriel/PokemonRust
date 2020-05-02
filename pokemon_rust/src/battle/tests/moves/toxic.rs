use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn toxic_causes_bad_poison() {
    let mut backend = battle! {
        "Koffing" 36 (max ivs, Serious) vs "Metapod" 36 (max ivs, Serious)
    };

    let events = backend.process_turn("Toxic", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition {
        target: 1,
        condition: StatusCondition::Toxic { counter: 1 },
    });
}

#[test]
fn toxic_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Koffing" 36 (max ivs, Serious) vs "Metapod" 36 (max ivs, Serious)
    };

    backend.process_turn("Toxic", "Harden");
    let events = backend.process_turn("Toxic", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
