use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods};

#[test]
fn vine_whip_deals_damage() {
    let mut backend = battle! {
        "Bulbasaur" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("VineWhip", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
