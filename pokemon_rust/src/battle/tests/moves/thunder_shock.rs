use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn thunder_shock_deals_damage_and_might_paralyze() {
    let mut backend = battle! {
        "Pikachu" 3 (max ivs, Serious) vs "Metapod" 3 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("ThunderShock", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn2 = backend.process_turn("ThunderShock", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, Some(StatusCondition::Paralysis));
}
