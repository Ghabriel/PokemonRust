use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn mega_punch_deals_damage() {
    let mut backend = battle! {
        "Hitmonchan" 32 (max ivs, Serious) vs "Charmander" 32 (max ivs, Serious)
    };

    let events = backend.process_turn("MachPunch", "Slash");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
