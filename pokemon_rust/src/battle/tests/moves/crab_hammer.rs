use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn crab_hammer_deals_damage_and_crits() {
    let mut backend = battle! {
        "Krabby" 44 (max ivs, Serious) vs "Metapod" 44 (max ivs, Serious)
    };

    let events = backend.process_turn("CrabHammer", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: true, .. });
}
