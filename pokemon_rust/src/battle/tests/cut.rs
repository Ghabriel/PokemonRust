use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods};

#[test]
fn cut_deals_damage() {
    let mut backend = battle! {
        "Farfetchd" 15 (max ivs, Serious) vs "Metapod" 15 (max ivs, Serious)
    };

    let events = backend.process_turn("Cut", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
