use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn rock_slide_deals_damage_and_might_flinch() {
    let mut backend = battle! {
        "Onix" 20 (max ivs, Bold) vs "Metapod" 30 (max ivs, Impish)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("RockSlide", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn1[2], VolatileStatusCondition { target: 1, added_flag: Flag::Flinch });
    assert_eq!(test_rng!(backend.rng).get_last_secondary_effect_check_chance(), Some(30));

    let turn2 = backend.process_turn("RockSlide", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn2[2], UseMove { move_user: 1, .. });
}
