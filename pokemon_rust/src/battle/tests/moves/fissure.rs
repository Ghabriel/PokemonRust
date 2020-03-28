use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn fissure_deals_ohko_damage() {
    let mut backend = battle! {
        "Diglett" 44 (max ivs, Serious) vs "Metapod" 44 (max ivs, Serious)
    };

    let events = backend.process_turn("Fissure", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, is_ohko: true, .. });
    assert_event!(events[2], Faint { target: 1, .. });
}
