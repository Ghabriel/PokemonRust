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
