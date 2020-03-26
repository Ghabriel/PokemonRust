use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn water_gun_deals_damage() {
    let mut backend = battle! {
        "Krabby" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let events = backend.process_turn("WaterGun", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
}
