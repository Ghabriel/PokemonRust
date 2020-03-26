use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn peck_deals_damage() {
    let mut backend = battle! {
        "Farfetchd" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("Peck", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
