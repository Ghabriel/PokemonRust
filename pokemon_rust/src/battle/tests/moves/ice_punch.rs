use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn ice_punch_deals_damage_and_might_freeze() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("IcePunch", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    test_rng_mut!(backend.rng).force_freeze_duration(1);
    let turn2 = backend.process_turn("IcePunch", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, Some(StatusCondition::Freeze));
}
