use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn pound_deals_damage() {
    let mut backend = battle! {
        "Clefairy" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("Pound", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
