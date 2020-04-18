use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn confuse_ray_causes_confusion() {
    let mut backend = battle! {
        "Lapras" 25 (max ivs, Serious) vs "Metapod" 25 (max ivs, Serious)
    };

    let events = backend.process_turn("ConfuseRay", "Harden");

    assert_event!(events[1], VolatileStatusCondition { target: 1, added_flag: Flag::Confusion { .. } });
    assert!(backend.has_flag(1, "confusion"));
}

#[test]
fn confuse_ray_fails_if_the_target_is_already_confused() {
    let mut backend = battle! {
        "Lapras" 25 (max ivs, Serious) vs "Metapod" 25 (max ivs, Serious)
    };

    backend.process_turn("ConfuseRay", "Harden");
    let events = backend.process_turn("ConfuseRay", "Harden");

    assert_event!(events[1], FailedMove { move_user: 0 });
}
