use crate::battle::backend::{BattleEvent, Team, TypeEffectiveness};

use super::{prelude::*, TestMethods};

#[test]
fn switches_in_all_participants_in_first_turn() {
    let mut backend = battle_setup!("Rattata" 3 vs "Pidgey" 3);
    let events: Vec<_> = backend.tick().collect();

    assert_event!(events[0], InitialSwitchIn { team: Team::P2, pokemon: 1, .. });
    assert_event!(events[1], InitialSwitchIn { team: Team::P1, pokemon: 0, .. });
}

#[test]
fn treats_wild_pokemon_as_already_sent_out() {
    let mut backend = battle_setup!("Rattata" 3 vs "Pidgey" 3);
    let events: Vec<_> = backend.tick().collect();

    assert_event!(events[0], InitialSwitchIn { is_already_sent_out: true, .. });
}

#[test]
fn treats_owned_pokemon_as_not_already_sent_out() {
    let p1 = pokemon_setup!("Rattata" 3);
    let p2 = pokemon_setup!("Pidgey" 3);
    let mut backend = create_simple_trainer_battle(p1, p2);
    let events: Vec<_> = backend.tick().collect();

    assert_event!(events[0], InitialSwitchIn { is_already_sent_out: false, .. });
}

#[test]
fn treats_the_player_pokemon_as_not_already_sent_out() {
    let mut backend = battle_setup!("Rattata" 3 vs "Pidgey" 3);
    let events: Vec<_> = backend.tick().collect();

    assert_event!(events[1], InitialSwitchIn { is_already_sent_out: false, .. });
}

#[test]
fn applies_type_effectiveness() {
    let mut backend = battle! {
        "Pidgey" 10 (max ivs, Adamant) vs "Hitmonchan" 10 (max ivs, Serious)
    };

    let events = backend.process_turn("Gust", "Tackle");

    assert_event!(events[1], Damage {
        target: 0,
        amount: 10,
        effectiveness: TypeEffectiveness::Normal,
        is_critical_hit: false,
    });
    assert_event!(events[3], Damage {
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

    match (&turn1[1], &turn2[1]) {
        (
            BattleEvent::Damage(Damage { amount: a1, .. }),
            BattleEvent::Damage(Damage { amount: a2, .. }),
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

    assert_event!(turn1[1], Damage { is_critical_hit: true, .. });
    assert_event!(turn1[3], Damage { is_critical_hit: false, .. });
    assert_event!(turn2[1], Damage { is_critical_hit: false, .. });
    assert_event!(turn2[3], Damage { is_critical_hit: false, .. });
}

#[test]
fn ignores_attack_debuffs_on_critical_hit() {
    let mut backend = battle! {
        "Charmander" 20 (max ivs, Naive) vs "Charmander" 20 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("Slash", "Growl");
    let turn2 = backend.process_turn("Slash", "Growl");

    match (&turn1[1], &turn2[1]) {
        (
            BattleEvent::Damage(Damage { amount: a1, .. }),
            BattleEvent::Damage(Damage { amount: a2, .. }),
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

    match (&turn1[1], &turn2[1]) {
        (
            BattleEvent::Damage(Damage { amount: a1, .. }),
            BattleEvent::Damage(Damage { amount: a2, .. }),
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

    match (&turn1[1], &turn3[1]) {
        (
            BattleEvent::Damage(Damage { amount: a1, .. }),
            BattleEvent::Damage(Damage { amount: a2, .. }),
        ) => {
            assert_eq!(*a2, ((*a1 as f32) * 2. / 3.) as usize);
        },
        _ => panic!("Pattern mismatch"),
    }
}
