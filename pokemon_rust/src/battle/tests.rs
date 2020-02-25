use crate::{
    battle::{
        backend::{BattleBackend, BattleEvent, FrontendEvent, FrontendEventKind, Team, UsedMove},
        rng::BattleRng,
        types::{Battle, BattleCharacterTeam, BattleType, Party},
    },
    entities::pokemon::{
        generator::{generate_pokemon, PokemonBuilder},
        get_all_moves,
        get_all_pokemon_species,
        Nature,
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

    fn build(self) -> BattleBackend<TestRng> {
        BattleBackend::new(
            Battle::new(self.battle_type, self.p1.unwrap(), self.p2.unwrap()),
            TestRng,
        )
    }
}

fn battle() -> BattleBuilder {
    BattleBuilder::new()
}

trait TestMethods {
    fn move_p1(&mut self, index: usize);
    fn move_p2(&mut self, index: usize);
}

impl<Rng: BattleRng> TestMethods for BattleBackend<Rng> {
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

#[derive(Debug)]
struct TestRng;

impl BattleRng for TestRng {
    fn get_damage_modifier(&mut self) -> f32 {
        1.
    }

    fn shuffle_moves<'a>(&mut self, _moves: &mut Vec<UsedMove<'a>>) {}
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

fn process_turn<'a, Rng: BattleRng>(
    backend: &'a mut BattleBackend<Rng>,
    p1_move: &str,
    p2_move: &str,
) -> impl Iterator<Item = BattleEvent> + 'a {
    let p1_index = backend.p1.active_pokemon.unwrap();
    let p2_index = backend.p2.active_pokemon.unwrap();

    let p1_move_index = backend
        .pokemon_repository[&p1_index]
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, mov)| match mov {
            Some(mov) => Some((i, mov)),
            None => None,
        })
        .find(|(i, mov)| mov.as_str() == p1_move)
        .map(|(i, _)| i)
        .unwrap();

    let p2_move_index = backend
        .pokemon_repository[&p2_index]
        .moves
        .iter()
        .enumerate()
        .filter_map(|(i, mov)| match mov {
            Some(mov) => Some((i, mov)),
            None => None,
        })
        .find(|(i, mov)| mov.as_str() == p2_move)
        .map(|(i, _)| i)
        .unwrap();

    backend.move_p1(p1_move_index);
    backend.move_p2(p2_move_index);
    backend.tick()
}

macro_rules! battle {
    (
        $p1_species:literal $p1_level:literal $(($($p1_data:tt)*))?
        vs
        $p2_species:literal $p2_level:literal $(($($p2_data:tt)*))?
    ) => {
        {
            let pokedex = get_all_pokemon_species();
            let movedex = get_all_moves();

            let p1_builder = PokemonBuilder::default();
            $(let p1_builder = constrain_pokemon!(p1_builder, $($p1_data)*);)?
            let p1 = p1_builder.build(
                &pokedex.get_species($p1_species).unwrap(),
                &movedex,
                $p1_level,
            );

            let p2_builder = PokemonBuilder::default();
            $(let p2_builder = constrain_pokemon!(p2_builder, $($p2_data)*);)?
            let p2 = p2_builder.build(
                &pokedex.get_species($p2_species).unwrap(),
                &movedex,
                $p2_level,
            );

            BattleBackend::new(
                Battle::new(
                    BattleType::Single,
                    BattleCharacterTeam {
                        active_pokemon: None,
                        party: Party { pokemon: vec![p1].into() },
                        character_id: None,
                    },
                    BattleCharacterTeam {
                        active_pokemon: None,
                        party: Party { pokemon: vec![p2].into() },
                        character_id: None,
                    },
                ),
                TestRng,
            )
        }
    }
}

macro_rules! constrain_pokemon {
    ($builder:ident, max ivs$(, $($data:tt)*)*) => {
        {
            let $builder = $builder.with_ivs([31; 6]);
            constrain_pokemon!($builder, $($($data)?)*)
        }
    };

    ($builder:ident, Hardy) => {
        $builder.with_nature(Nature::Hardy)
    };

    ($builder:ident, Lonely) => {
        $builder.with_nature(Nature::Lonely)
    };

    ($builder:ident, Adamant) => {
        $builder.with_nature(Nature::Adamant)
    };

    ($builder:ident, Naughty) => {
        $builder.with_nature(Nature::Naughty)
    };

    ($builder:ident, Brave) => {
        $builder.with_nature(Nature::Brave)
    };

    ($builder:ident, Bold) => {
        $builder.with_nature(Nature::Bold)
    };

    ($builder:ident, Docile) => {
        $builder.with_nature(Nature::Docile)
    };

    ($builder:ident, Impish) => {
        $builder.with_nature(Nature::Impish)
    };

    ($builder:ident, Lax) => {
        $builder.with_nature(Nature::Lax)
    };

    ($builder:ident, Relaxed) => {
        $builder.with_nature(Nature::Relaxed)
    };

    ($builder:ident, Modest) => {
        $builder.with_nature(Nature::Modest)
    };

    ($builder:ident, Mild) => {
        $builder.with_nature(Nature::Mild)
    };

    ($builder:ident, Bashful) => {
        $builder.with_nature(Nature::Bashful)
    };

    ($builder:ident, Rash) => {
        $builder.with_nature(Nature::Rash)
    };

    ($builder:ident, Quiet) => {
        $builder.with_nature(Nature::Quiet)
    };

    ($builder:ident, Calm) => {
        $builder.with_nature(Nature::Calm)
    };

    ($builder:ident, Gentle) => {
        $builder.with_nature(Nature::Gentle)
    };

    ($builder:ident, Careful) => {
        $builder.with_nature(Nature::Careful)
    };

    ($builder:ident, Quirky) => {
        $builder.with_nature(Nature::Quirky)
    };

    ($builder:ident, Sassy) => {
        $builder.with_nature(Nature::Sassy)
    };

    ($builder:ident, Timid) => {
        $builder.with_nature(Nature::Timid)
    };

    ($builder:ident, Hasty) => {
        $builder.with_nature(Nature::Hasty)
    };

    ($builder:ident, Jolly) => {
        $builder.with_nature(Nature::Jolly)
    };

    ($builder:ident, Naive) => {
        $builder.with_nature(Nature::Naive)
    };

    ($builder:ident, Serious) => {
        $builder.with_nature(Nature::Serious)
    };
}

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
            assert_ne!(t1, t2);
            assert_eq!(a1, 28);
            assert_eq!(a2, 22);
        },
        _ => panic!("Wrong event kind"),
    }
}
