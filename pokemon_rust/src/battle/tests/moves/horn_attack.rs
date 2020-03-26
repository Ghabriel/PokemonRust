use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn horn_attack_deals_damage() {
    let mut backend = battle! {
        "Rhyhorn" 15 (max ivs, Serious) vs "Metapod" 10 (max ivs, Serious)
    };

    let events = backend.process_turn("HornAttack", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
