use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn hypnosis_causes_sleep() {
    let mut backend = battle! {
        "Exeggcutor" 5 (max ivs, Serious) vs "Metapod" 5 (max ivs, Serious)
    };

    let events = backend.process_turn("Hypnosis", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition { target: 1, condition: StatusCondition::Sleep { .. } });
}

#[test]
fn hypnosis_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Exeggcutor" 5 (max ivs, Serious) vs "Metapod" 5 (max ivs, Serious)
    };

    backend.process_turn("Hypnosis", "Harden");
    let events = backend.process_turn("Hypnosis", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
