use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn double_kick_deals_damage_twice() {
    let mut backend = battle! {
        "Hitmonlee" 4 (max ivs, Serious) vs "Metapod" 4 (max ivs, Serious)
    };

    let events = backend.process_turn("DoubleKick", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(events[2], Damage { target: 1, is_critical_hit: false, .. });
}
