use crate::battle::backend::{BattleEvent, StatChangeKind};

use super::super::{prelude::*, TestMethods};

#[test]
fn acid_damage_and_might_reduce_speed() {
    let mut backend = battle! {
        "Oddish" 4 (max ivs, Impish) vs "Metapod" 10 (max ivs, Impish)
    };
    backend.get_pokemon_mut(1).stats[5] = 0;

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("Acid", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn1[2], StatChange { target: 1, kind: StatChangeKind::Fell, stat: Stat::SpecialDefense });

    let turn2 = backend.process_turn("Acid", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn2[2], UseMove { move_user: 1, .. });
}
