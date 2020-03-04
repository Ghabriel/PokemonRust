use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn karate_chop_deals_damage_and_crits() {
    let mut backend = battle! {
        "Machop" 7 (max ivs, Serious) vs "Metapod" 7 (max ivs, Serious)
    };

    let events = backend.process_turn("KarateChop", "Harden");

    assert_event!(events[0], Damage { target: 1, is_critical_hit: true, .. });
}
