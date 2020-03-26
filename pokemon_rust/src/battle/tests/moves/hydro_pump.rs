use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn hydro_pump_deals_damage() {
    let mut backend = battle! {
        "Squirtle" 33 (max ivs, Serious) vs "Metapod" 33 (max ivs, Serious)
    };

    let events = backend.process_turn("HydroPump", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
