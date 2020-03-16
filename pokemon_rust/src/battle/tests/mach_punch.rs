use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn mach_punch_deals_damage_and_has_increased_priority() {
    let mut backend = battle! {
        "Hitmonchan" 4 (max ivs, Serious) vs "Rattata" 100 (max ivs, Serious)
    };

    let events = backend.process_turn("MachPunch", "Bite");

    assert_event!(events[0], Damage { target: 1, is_critical_hit: false, .. });
}
