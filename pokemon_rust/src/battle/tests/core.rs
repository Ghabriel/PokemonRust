use crate::{
    battle::backend::{BattleEvent, StatChangeKind, Team, TypeEffectiveness},
    pokemon::SimpleStatusCondition,
};

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
        ..
    });
    assert_event!(events[3], Damage {
        target: 1,
        amount: 12,
        effectiveness: TypeEffectiveness::SuperEffective,
        is_critical_hit: false,
        ..
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
    let turn2 = backend.process_turn("Ember", "Tackle");

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
            assert_eq!(*a1, ((*a2 as f32) * 2. / 3.) as usize);
        },
        _ => panic!("Pattern mismatch"),
    }
}

#[test]
fn causes_random_misses_when_confused() {
    let mut backend = battle! {
        "Butterfree" 4 (max ivs, Serious) vs "Caterpie" 4 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_confusion_miss(2);
    let turn1 = backend.process_turn("Supersonic", "Tackle");
    let turn2 = backend.process_turn("Harden", "Tackle");
    let turn3 = backend.process_turn("Harden", "Tackle");

    assert_event!(turn1[3], Miss { target: 0, move_user: 1, caused_by_confusion: true, .. });
    assert_event!(turn2[3], Miss { target: 0, move_user: 1, caused_by_confusion: true, .. });
    assert_event!(turn3[3], Damage { target: 0, .. });
}

#[test]
fn makes_confusion_expire() {
    let mut backend = battle! {
        "Butterfree" 4 (max ivs, Serious) vs "Caterpie" 4 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_confusion_duration(1);
    backend.process_turn("Supersonic", "Tackle");
    let events = backend.process_turn("Harden", "Tackle");

    assert_event!(events[2], ExpiredVolatileStatusCondition {
        target: 1,
        flag: Flag::Confusion { remaining_move_attempts: 0 },
        ..
    });
}

#[test]
fn deals_damage_to_burned_pokemon() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Krabby" 24 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let events = backend.process_turn("FirePunch", "Harden");
    assert_event!(events[5], Damage { target: 1, .. });
}

#[test]
fn halves_physical_damage_of_burned_pokemon() {
    let mut backend = battle! {
        "Hitmonchan" 36 (max ivs, Serious) vs "Krabby" 36 (max ivs, Serious)
    };

    let turn1 = backend.process_turn("FirePunch", "Slam");
    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn2 = backend.process_turn("FirePunch", "Slam");

    match (&turn1[3], &turn2[4]) {
        (
            BattleEvent::Damage(Damage { amount: a1, .. }),
            BattleEvent::Damage(Damage { amount: a2, .. }),
        ) => {
            assert_eq!(*a2, *a1 / 2);
        },
        _ => panic!("Pattern mismatch"),
    }
}

#[test]
fn fire_types_cannot_be_burned() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Charmander" 24 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let events = backend.process_turn("FirePunch", "Slash");
    assert_event!(events[1], Damage { target: 1, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);
}

#[test]
fn deals_damage_to_poisoned_pokemon() {
    let mut backend = battle! {
        "Weedle" 5 (max ivs, Serious) vs "Metapod" 5 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let events = backend.process_turn("PoisonSting", "Harden");
    assert_event!(events[5], Damage { target: 1, .. });
}

#[test]
fn poison_types_cannot_be_poisoned() {
    let mut backend = battle! {
        "Koffing" 8 (max ivs, Serious) vs "Bulbasaur" 5 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("Smog", "Growl");
    assert_event!(turn1[1], Damage { target: 1, .. });
    assert_eq!(backend.get_pokemon(0).status_condition, None);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn2 = backend.process_turn("PoisonGas", "Growl");
    assert_event!(turn2[1], FailedMove { move_user: 0, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);
}

#[test]
fn might_prevent_moves_of_paralyzed_pokemon() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    test_rng_mut!(backend.rng).force_paralysis_move_prevention(1);
    let turn1 = backend.process_turn("ThunderPunch", "Harden");
    assert_event!(turn1[4], FailedMove { move_user: 1, .. });

    let turn2 = backend.process_turn("ThunderPunch", "Harden");
    assert_event!(turn2[3], StatChange { target: 1, kind: StatChangeKind::Rose, stat: Stat::Defense });
}

#[test]
fn halves_speed_of_paralyzed_pokemon() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    let base_speed = backend.get_stat(1, Stat::Speed);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    backend.process_turn("ThunderPunch", "Harden");

    let new_speed = backend.get_stat(1, Stat::Speed);

    assert_eq!(new_speed, base_speed / 2);
}

#[test]
fn electric_types_cannot_be_paralyzed() {
    let mut backend = battle! {
        "Pikachu" 4 (max ivs, Serious) vs "Pikachu" 3 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn1 = backend.process_turn("Growl", "ThunderShock");
    assert_event!(turn1[3], Damage { target: 0, .. });
    assert_eq!(backend.get_pokemon(0).status_condition, None);

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let turn2 = backend.process_turn("ThunderWave", "Growl");
    assert_event!(turn2[1], FailedMove { move_user: 0, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);
}

#[test]
fn prevents_moves_of_frozen_pokemon() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    test_rng_mut!(backend.rng).force_freeze_duration(1);
    let turn1 = backend.process_turn("IcePunch", "Harden");
    assert_event!(turn1[3], FailedMove { move_user: 1, .. });

    let turn2 = backend.process_turn("IcePunch", "Harden");
    assert_event!(turn2[4], StatChange { target: 1, kind: StatChangeKind::Rose, stat: Stat::Defense });
}

#[test]
fn makes_freeze_expire() {
    let mut backend = battle! {
        "Hitmonchan" 24 (max ivs, Serious) vs "Metapod" 24 (max ivs, Serious)
    };

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    test_rng_mut!(backend.rng).force_freeze_duration(1);
    backend.process_turn("IcePunch", "Harden");
    let events = backend.process_turn("IcePunch", "Harden");

    assert_event!(events[2], ExpiredNonVolatileStatusCondition {
        target: 1,
        condition: SimpleStatusCondition::Freeze,
        ..
    });
}

#[test]
fn ice_types_cannot_be_frozen() {
    let mut backend = battle! {
        "Lapras" 45 (max ivs, Serious) vs "Jynx" 45 (max ivs, Serious)

    };
    backend.get_pokemon_mut(1).stats[5] = 0;

    test_rng_mut!(backend.rng).force_secondary_effect(1);
    let events = backend.process_turn("IceBeam", "Psychic");
    assert_event!(events[1], Damage { target: 1, .. });
    assert_eq!(backend.get_pokemon(1).status_condition, None);
}
