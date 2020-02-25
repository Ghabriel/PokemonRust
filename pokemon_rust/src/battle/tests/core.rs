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

use super::{process_turn, TestRng};
// use super::process_turn;

// use crate::{battle, constrain_pokemon};

#[test]
fn switches_in_all_participants_in_first_turn() {
    let mut backend = battle!("Rattata" 3 vs "Pidgey" 3);
    let mut events = backend.tick();

    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P1, 0));
    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P2, 1));
}

#[test]
fn tackle_deals_damage() {
    let mut backend = battle! {
        "Rattata" 50 (max ivs, Adamant) vs "Pidgey" 50 (max ivs, Adamant)
    };
    backend.tick();

    let mut events = process_turn(&mut backend, "Tackle", "Tackle");
    let first = events.next().unwrap();
    let second = events.next().unwrap();

    match (first, second) {
        (
            BattleEvent::Damage { target: t1, amount: a1 },
            BattleEvent::Damage { target: t2, amount: a2 },
        ) => {
            assert_eq!(t1, 1);
            assert_eq!(t2, 0);
            assert_eq!(a1, 28);
            assert_eq!(a2, 22);
        },
        _ => panic!("Wrong event kind"),
    }
}
