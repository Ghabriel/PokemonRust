use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn bite_deals_damage() {
    let mut backend = battle! {
        "Rattata" 12 (max ivs, Serious) vs "Metapod" 12 (max ivs, Serious)
    };

    let events = backend.process_turn("Bite", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
