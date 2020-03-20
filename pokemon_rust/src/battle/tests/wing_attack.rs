use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods};

#[test]
fn wing_attack_deals_damage() {
    let mut backend = battle! {
        "Pidgey" 33 (max ivs, Serious) vs "Metapod" 33 (max ivs, Serious)
    };

    let events = backend.process_turn("WingAttack", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
