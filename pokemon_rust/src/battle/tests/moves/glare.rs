use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn glare_causes_paralysis() {
    let mut backend = battle! {
        "Ekans" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    let events = backend.process_turn("Glare", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition { target: 1, condition: StatusCondition::Paralysis });
}

#[test]
fn glare_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Ekans" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    backend.process_turn("Glare", "Harden");
    let events = backend.process_turn("Glare", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
