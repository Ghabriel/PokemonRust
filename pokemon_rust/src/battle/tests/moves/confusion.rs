use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn confusion_deals_damage_and_might_cause_confusion() {
    let mut backend = battle! {
        "Butterfree" 8 (max ivs, Serious) vs "Metapod" 8 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    test_rng_mut!(backend.rng).force_confusion_duration(1);
    let turn1 = backend.process_turn("Confusion", "Harden");
    assert_event!(turn1[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn1[2], VolatileStatusCondition { target: 1, added_flag: Flag::Confusion { .. } });

    let turn2 = backend.process_turn("Confusion", "Harden");
    assert_event!(turn2[1], Damage { target: 1, is_critical_hit: false, .. });
    assert_event!(turn2[3], UseMove { move_user: 1, .. });
}
