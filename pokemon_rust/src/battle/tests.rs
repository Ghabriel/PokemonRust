use crate::{
    battle::{
        backend::{BattleBackend, BattleEvent, FrontendEvent, FrontendEventKind, Team},
        types::{Battle, BattleCharacterTeam, BattleType, Party},
    },
    entities::pokemon::{
        generator::generate_pokemon,
        get_all_moves,
        get_all_pokemon_species,
        Pokemon,
    },
};

struct BattleBuilder {
    battle_type: BattleType,
    p1: Option<BattleCharacterTeam>,
    p2: Option<BattleCharacterTeam>,
}

impl BattleBuilder {
    fn new() -> BattleBuilder {
        BattleBuilder {
            battle_type: BattleType::Single,
            p1: None,
            p2: None,
        }
    }

    fn p1(mut self, species_id: &str, level: usize) -> Self {
        self.p1 = Some(simple_team(species_id, level));
        self
    }

    fn p2(mut self, species_id: &str, level: usize) -> Self {
        self.p2 = Some(simple_team(species_id, level));
        self
    }

    fn build(self) -> BattleBackend {
        BattleBackend::new(Battle::new(
            self.battle_type,
            self.p1.unwrap(),
            self.p2.unwrap(),
        ))
    }
}

fn battle() -> BattleBuilder {
    BattleBuilder::new()
}

trait TestMethods {
    fn move_p1(&mut self, index: usize);
    fn move_p2(&mut self, index: usize);
}

impl TestMethods for BattleBackend {
    fn move_p1(&mut self, index: usize) {
        self.push_frontend_event(FrontendEvent {
            team: Team::P1,
            event: FrontendEventKind::UseMove(index),
        });
    }

    fn move_p2(&mut self, index: usize) {
        self.push_frontend_event(FrontendEvent {
            team: Team::P2,
            event: FrontendEventKind::UseMove(index),
        });
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

#[test]
fn switches_in_all_participants_in_first_turn() {
    let mut backend = battle().p1("Rattata", 3).p2("Pidgey", 3).build();
    let mut events = backend.tick();

    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P1, 0));
    assert_eq!(events.next().unwrap(), BattleEvent::InitialSwitchIn(Team::P2, 1));
}

#[test]
fn tackle_deals_damage() {
    let mut backend = battle().p1("Rattata", 3).p2("Pidgey", 3).build();

    backend.tick();
    backend.move_p1(0);
    backend.move_p2(0);

    let mut events = backend.tick();
    let first = events.next().unwrap();
    let second = events.next().unwrap();

    match (first, second) {
        (
            BattleEvent::Damage { target: t1, amount: a1 },
            BattleEvent::Damage { target: t2, amount: a2 },
        ) => {
            assert_ne!(t1, t2);
            assert!(a1 >= 3 && a1 <= 5);
            assert!(a2 >= 3 && a2 <= 5);
        },
        _ => panic!("Wrong event kind"),
    }
}
