use crate::battle::backend::BattleEvent;

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn tackle_deals_damage() {
    let mut backend = battle! {
        "Rattata" 50 (max ivs, Adamant) vs "Pidgey" 50 (max ivs, Adamant)
    };

    let events = backend.process_turn("Tackle", "Tackle");

    assert_event!(events[0], Damage {
        target: 1,
        amount: 39,
        is_critical_hit: false,
        ..
    });
    assert_event!(events[1], Damage {
        target: 0,
        amount: 36,
        is_critical_hit: false,
        ..
    });
}

#[test]
fn tackle_does_accuracy_checks() {
    let mut backend = battle! {
        "Rattata" 3 (max ivs, Serious) vs "Pidgey" 3 (max ivs, Serious)
    };

    backend.rng.force_miss(3);
    let turn1 = backend.process_turn("Tackle", "Tackle");
    let turn2 = backend.process_turn("Tackle", "Tackle");

    assert_event!(turn1[0], Miss { move_user: 0 });
    assert_event!(turn1[1], Miss { move_user: 1 });
    assert_event!(turn2[0], Miss { move_user: 0 });
    assert_event!(turn2[1], Damage { target: 0, .. });
    assert_eq!(backend.rng.get_last_miss_check_chance(), Some(100));
}
