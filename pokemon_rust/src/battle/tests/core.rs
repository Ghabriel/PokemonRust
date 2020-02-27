use crate::battle::backend::{BattleEvent, Team, TypeEffectiveness};

use super::{prelude::*, TestMethods, TestRng};

#[test]
fn switches_in_all_participants_in_first_turn() {
    let mut backend = battle_setup!("Rattata" 3 vs "Pidgey" 3);
    let mut events = backend.tick();

    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P1, 0));
    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P2, 1));
}

#[test]
fn applies_type_effectiveness() {
    let mut backend = battle! {
        "Pidgey" 10 (max ivs, Adamant) vs "Hitmonchan" 10 (max ivs, Serious)
    };

    let events = backend.process_turn("Gust", "Tackle");

    assert_eq!(events[0], BattleEvent::Damage {
        target: 0,
        amount: 10,
        effectiveness: TypeEffectiveness::Normal,
        is_critical_hit: false,
    });
    assert_eq!(events[1], BattleEvent::Damage {
        target: 1,
        amount: 12,
        effectiveness: TypeEffectiveness::SuperEffective,
        is_critical_hit: false,
    });
}

#[test]
fn applies_stab() {
    let mut backend = battle! {
        "Hitmonchan" 10 (max ivs, Adamant) vs "Pidgey" 10 (max ivs, Adamant)
    };

    let turn1 = backend.process_turn("MachPunch", "Tackle");
    let turn2 = backend.process_turn("Tackle", "Tackle");

    match (&turn1[0], &turn2[0]) {
        (
            BattleEvent::Damage { amount: a1, .. },
            BattleEvent::Damage { amount: a2, .. },
        ) => {
            assert_eq!(*a1, (1.5 * *a2 as f32) as usize);
        },
        _ => panic!("Pattern mismatch"),
    }
}

#[test]
fn applies_critical_hit() {
    let mut backend = battle! {
        "Charmander" 20 (max ivs, Serious) vs "Pidgey" 20 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Slash", "Tackle");
    let turn2 = backend.process_turn("Scratch", "Tackle");

    assert_pattern!(turn1[0], BattleEvent::Damage { is_critical_hit: true, .. });
    assert_pattern!(turn1[1], BattleEvent::Damage { is_critical_hit: false, .. });
    assert_pattern!(turn2[0], BattleEvent::Damage { is_critical_hit: false, .. });
    assert_pattern!(turn2[1], BattleEvent::Damage { is_critical_hit: false, .. });
}

#[test]
fn ignores_attack_debuffs_on_critical_hit() {
    let mut backend = battle! {
        "Charmander" 20 (max ivs, Naive) vs "Charmander" 20 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Slash", "Growl");
    let turn2 = backend.process_turn("Slash", "Growl");

    match (&turn1[0], &turn2[0]) {
        (
            BattleEvent::Damage { amount: a1, .. },
            BattleEvent::Damage { amount: a2, .. },
        ) => {
            assert_eq!(*a1, *a2);
        },
        _ => panic!("Pattern mismatch"),
    }
}

#[test]
fn ignores_defense_buffs_on_critical_hit() {
    let mut backend = battle! {
        "Charmander" 20 (max ivs, Serious) vs "Metapod" 20 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Slash", "Harden");
    let turn2 = backend.process_turn("Slash", "Harden");

    match (&turn1[0], &turn2[0]) {
        (
            BattleEvent::Damage { amount: a1, .. },
            BattleEvent::Damage { amount: a2, .. },
        ) => {
            assert_eq!(*a1, *a2);
        },
        _ => panic!("Pattern mismatch"),
    }
}

#[test]
fn considers_stat_stages_on_damage_calculation() {
    let mut backend = battle! {
        "Rattata" 3 (max ivs, Serious) vs "Pidgey" 3 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Tackle", "Tackle");
    backend.process_turn("TailWhip", "Tackle");
    let turn3 = backend.process_turn("Tackle", "Tackle");

    match (&turn1[0], &turn3[0]) {
        (
            BattleEvent::Damage { amount: a1, .. },
            BattleEvent::Damage { amount: a2, .. },
        ) => {
            assert_eq!(*a2, ((*a1 as f32) * 2. / 3.) as usize);
        },
        _ => panic!("Pattern mismatch"),
    }
}
