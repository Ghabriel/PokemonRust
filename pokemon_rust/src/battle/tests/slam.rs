use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn slam_deals_damage() {
    let mut backend = battle! {
        "Krabby" 36 (max ivs, Serious) vs "Metapod" 36 (max ivs, Serious)
    };

    let events = backend.process_turn("Slam", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
