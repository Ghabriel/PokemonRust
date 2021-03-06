use crate::battle::backend::BattleEvent;

use super::super::{prelude::*, TestMethods};

#[test]
fn tackle_deals_damage() {
    let mut backend = battle! {
        "Rattata" 5 (max ivs, Adamant) vs "Pidgey" 5 (max ivs, Adamant)
    };

    let events = backend.process_turn("Tackle", "Tackle");

    assert_event!(events[1], Damage {
        target: 1,
        amount: 9,
        is_critical_hit: false,
        ..
    });
    assert_event!(events[3], Damage {
        target: 0,
        amount: 7,
        is_critical_hit: false,
        ..
    });
}

#[test]
fn tackle_does_accuracy_checks() {
    let mut backend = battle! {
        "Rattata" 3 (max ivs, Serious) vs "Pidgey" 3 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_miss(3);
    let turn1 = backend.process_turn("Tackle", "Tackle");
    let turn2 = backend.process_turn("Tackle", "Tackle");

    assert_event!(turn1[1], Miss { target: 1, move_user: 0, .. });
    assert_event!(turn1[3], Miss { target: 0, move_user: 1, .. });
    assert_event!(turn2[1], Miss { target: 1, move_user: 0, .. });
    assert_event!(turn2[3], Damage { target: 0, .. });
    assert_eq!(test_rng!(backend.rng).get_last_miss_check_chance(), Some(100));
}
