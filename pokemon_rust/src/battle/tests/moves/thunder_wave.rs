use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn thunder_wave_causes_paralysis() {
    let mut backend = battle! {
        "Pikachu" 4 (max ivs, Serious) vs "Metapod" 4 (max ivs, Serious)
    };

    let events = backend.process_turn("ThunderWave", "Harden");

    assert_event!(events[1], NonVolatileStatusCondition { target: 1, condition: StatusCondition::Paralysis });
}

#[test]
fn thunder_wave_fails_if_the_target_already_has_non_volatile_condition() {
    let mut backend = battle! {
        "Pikachu" 4 (max ivs, Serious) vs "Metapod" 4 (max ivs, Serious)
    };

    backend.process_turn("ThunderWave", "Harden");
    let events = backend.process_turn("ThunderWave", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
