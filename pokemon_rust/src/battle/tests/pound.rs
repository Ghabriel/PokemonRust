use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn pound_deals_damage() {
    let mut backend = battle! {
        "Clefairy" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("Pound", "Harden");

    assert_pattern!(events[0], BattleEvent::Damage { target: 1, is_critical_hit: false, .. });
}
