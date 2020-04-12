use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn poison_powder_causes_poison() {
    let mut backend = battle! {
        "Butterfree" 12 (max ivs, Serious) vs "Metapod" 12 (max ivs, Serious)
    };

    let events = backend.process_turn("PoisonPowder", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition { target: 1, condition: StatusCondition::Poison });
}

#[test]
fn poison_powder_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Butterfree" 12 (max ivs, Serious) vs "Metapod" 12 (max ivs, Serious)
    };

    backend.process_turn("PoisonPowder", "Harden");
    let events = backend.process_turn("PoisonPowder", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
