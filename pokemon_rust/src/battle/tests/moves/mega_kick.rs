use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn mega_kick_deals_damage() {
    let mut backend = battle! {
        "Hitmonlee" 32 (max ivs, Serious) vs "Metapod" 32 (max ivs, Serious)
    };

    let events = backend.process_turn("MegaKick", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
