use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn drill_peck_deals_damage() {
    let mut backend = battle! {
        "Spearow" 32 (max ivs, Serious) vs "Metapod" 32 (max ivs, Serious)
    };

    let events = backend.process_turn("DrillPeck", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
