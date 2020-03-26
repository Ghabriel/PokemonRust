use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn razor_leaf_deals_damage_and_crits() {
    let mut backend = battle! {
        "Bulbasaur" 12 (max ivs, Serious) vs "Metapod" 12 (max ivs, Serious)
    };

    let events = backend.process_turn("RazorLeaf", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: true, .. });
}
