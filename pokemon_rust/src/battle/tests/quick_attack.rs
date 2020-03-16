use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn quick_attack_deals_damage_and_has_increased_priority() {
    let mut backend = battle! {
        "Rattata" 6 (max ivs, Serious) vs "Hitmonchan" 100 (max ivs, Serious)
    };

    let events = backend.process_turn("QuickAttack", "MegaPunch");

    assert_event!(events[0], Damage { target: 1, is_critical_hit: false, .. });
}
