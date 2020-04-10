use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn bone_club_deals_damage_and_might_flinch() {
    let mut backend = battle! {
        "Cubone" 12 (max ivs, Serious) vs "Metapod" 12 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("BoneClub", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn1[2], VolatileStatusCondition { target: 1, added_flag: Flag::Flinch });
    assert_eq!(test_rng!(backend.rng).get_last_secondary_effect_check_chance(), Some(10));

    let turn2 = backend.process_turn("BoneClub", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn2[2], UseMove { move_user: 1, .. });
}
