use crate::{
    battle::backend::BattleEvent,
    pokemon::StatusCondition,
};

use super::super::{prelude::*, TestMethods};

#[test]
fn sludge_deals_damage_and_might_burn() {
    let mut backend = battle! {
        "Koffing" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Sludge", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn2 = backend.process_turn("Sludge", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, Some(StatusCondition::Poison));
}
