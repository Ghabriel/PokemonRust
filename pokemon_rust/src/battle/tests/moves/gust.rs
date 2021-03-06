use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn gust_deals_damage() {
    let mut backend = battle! {
        "Pidgey" 9 (max ivs, Serious) vs "Metapod" 9 (max ivs, Serious)
    };

    let events = backend.process_turn("Gust", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
