use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods};

#[test]
fn slash_deals_damage_and_crits() {
    let mut backend = battle! {
        "Charmander" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    let events = backend.process_turn("Slash", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: true, .. });
}
