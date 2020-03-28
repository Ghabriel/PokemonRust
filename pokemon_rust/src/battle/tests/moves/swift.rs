use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn swift_deals_damage_and_never_misses() {
    let mut backend = battle! {
        "Eevee" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    test_rng_mut(&mut backend.rng).force_miss(1);
    let events = backend.process_turn("Swift", "Harden");

    assert_event!(events[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(test_rng_mut(&mut backend.rng).get_last_miss_check_chance(), None);
}
