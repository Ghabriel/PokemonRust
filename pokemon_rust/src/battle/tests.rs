use crate::{
    battle::{
        backend::{BattleBackend, BattleEvent, Team},
        types::{Battle, BattleCharacterTeam, BattleType, Party},
    },
    entities::pokemon::{
        generator::generate_pokemon,
        get_all_moves,
        get_all_pokemon_species,
        Pokemon,
    },
};

#[test]
fn switches_in_all_participants_in_first_turn() {
    let battle_type = BattleType::Single;
    let p1 = simple_team("Rattata", 3);
    let p2 = simple_team("Pidgey", 3);
    let mut backend = BattleBackend::new(Battle::new(battle_type, p1, p2));

    let mut events = backend.tick();

    match events.next().unwrap() {
        BattleEvent::InitialSwitchIn(team, pokemon) => {
            assert_eq!(team, Team::P1);
            assert_eq!(pokemon.species_id, "Rattata");
        },
        _ => panic!("Wrong event kind"),
    }

    match events.next().unwrap() {
        BattleEvent::InitialSwitchIn(team, pokemon) => {
            assert_eq!(team, Team::P2);
            assert_eq!(pokemon.species_id, "Pidgey");
        },
        _ => panic!("Wrong event kind"),
    }
}

fn simple_team(species_id: &str, level: usize) -> BattleCharacterTeam {
    let pokemon = get_pokemon(species_id, level);

    BattleCharacterTeam {
        active_pokemon: None,
        party: Party { pokemon: vec![pokemon].into() },
        character_id: None,
    }
}

fn get_pokemon(species_id: &str, level: usize) -> Pokemon {
    let pokedex = get_all_pokemon_species();
    let movedex = get_all_moves();

    generate_pokemon(
        &pokedex.get_species(species_id).unwrap(),
        &movedex,
        level,
    )
}
