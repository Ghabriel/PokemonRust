// use crate::battle::backend::{BattleEvent, Team};

use crate::{
    battle::{
        backend::{BattleBackend, BattleEvent, Team},
        types::{Battle, BattleCharacterTeam, BattleType, Party},
    },
    entities::pokemon::{
        generator::PokemonBuilder,
        get_all_moves,
        get_all_pokemon_species,
        Nature,
    },
};

use super::{TestMethods, TestRng};

#[test]
fn switches_in_all_participants_in_first_turn() {
    let mut backend = battle_setup!("Rattata" 3 vs "Pidgey" 3);
    let mut events = backend.tick();

    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P1, 0));
    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P2, 1));
}

#[test]
fn tackle_deals_damage() {
    let mut backend = battle! {
        "Rattata" 50 (max ivs, Adamant) vs "Pidgey" 50 (max ivs, Adamant)
    };

    let events = backend.process_turn("Tackle", "Tackle");

    assert_eq!(events[0], BattleEvent::Damage { target: 1, amount: 28 });
    assert_eq!(events[1], BattleEvent::Damage { target: 0, amount: 22 });
}

#[test]
fn tackle_does_accuracy_checks() {
    let mut backend = battle! {
        "Rattata" 3 vs "Pidgey" 3
    };

    backend.rng.force_miss(3);
    let turn1 = backend.process_turn("Tackle", "Tackle");
    let turn2 = backend.process_turn("Tackle", "Tackle");

    assert_eq!(turn1[0], BattleEvent::Miss(0));
    assert_eq!(turn1[1], BattleEvent::Miss(1));
    assert_eq!(turn2[0], BattleEvent::Miss(0));
    assert_pattern!(turn2[1], BattleEvent::Damage { target: 0, .. });
    assert_eq!(backend.rng.get_last_miss_check_chance(), Some(100));
}
