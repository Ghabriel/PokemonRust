use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn strength_deals_damage() {
    let mut backend = battle! {
        "Machop" 29 (max ivs, Serious) vs "Metapod" 29 (max ivs, Serious)
    };

    let events = backend.process_turn("Strength", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
