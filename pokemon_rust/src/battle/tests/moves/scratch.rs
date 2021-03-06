use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn scratch_deals_damage() {
    let mut backend = battle! {
        "Charmander" 1 (max ivs, Serious) vs "Metapod" 1 (max ivs, Serious)
    };

    let events = backend.process_turn("Scratch", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
