use crate::{
    constants::MOVE_LIMIT,
    entities::pokemon::{
        movement::MoveDex,
        Gender,
        LearningCondition,
        Nature,
        Pokemon,
        PokemonSpeciesData,
        PokerusData,
    },
};

use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};

use std::time::SystemTime;

/// Allows creation of a Pokémon while fixing some parameters and automatically
/// calculating others.
#[derive(Default)]
pub struct PokemonBuilder {
    nature: Option<Nature>,
    held_item: Option<String>,
    ability: Option<String>,
    evs: Option<[usize; 6]>,
    natural_ivs: Option<[usize; 6]>,
    moves: Option<[Option<String>; MOVE_LIMIT]>,
    pp: Option<[usize; MOVE_LIMIT]>,
    gender: Option<Gender>,
    // shiny: bool,

    // Battle stats
    stats: Option<[usize; 6]>,
}

impl PokemonBuilder {
    pub fn with_nature(mut self, nature: Nature) -> Self {
        self.nature = Some(nature);
        self
    }

    pub fn with_held_item(mut self, item: String) -> Self {
        self.held_item = Some(item);
        self
    }

    pub fn with_ability(mut self, ability: String) -> Self {
        self.ability = Some(ability);
        self
    }

    pub fn with_evs(mut self, evs: [usize; 6]) -> Self {
        self.evs = Some(evs);
        self
    }

    pub fn with_ivs(mut self, ivs: [usize; 6]) -> Self {
        self.natural_ivs = Some(ivs);
        self
    }

    pub fn with_moves(mut self, moves: [Option<String>; MOVE_LIMIT]) -> Self {
        self.moves = Some(moves);
        self
    }

    pub fn with_pp(mut self, pp: [usize; MOVE_LIMIT]) -> Self {
        self.pp = Some(pp);
        self
    }

    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn with_stats(mut self, stats: [usize; 6]) -> Self {
        self.stats = Some(stats);
        self
    }

    pub fn build(
        self,
        species_data: &PokemonSpeciesData,
        move_dex: &MoveDex,
        level: usize,
    ) -> Pokemon {
        let nature = self.nature.unwrap_or_else(pick_nature);
        let evs = self.evs.unwrap_or([0, 0, 0, 0, 0, 0]);
        let ivs = self.natural_ivs.unwrap_or_else(pick_ivs);
        let moves = self.moves.unwrap_or_else(|| {
            pick_moves(&species_data.move_table, level)
        });
        let pp = self.pp.unwrap_or_else(|| pick_pps(&move_dex, &moves));
        let stats = self.stats.unwrap_or_else(|| {
            pick_stats(&species_data.base_stats, &evs, &ivs, nature, level)
        });

        Pokemon {
            species_id: species_data.id.clone(),
            nature,
            held_item: self.held_item,
            experience_points: 0,
            ability: self.ability.unwrap_or_else(|| pick_ability(&species_data.abilities)),
            evs,
            natural_ivs: ivs,
            obtained_ivs: [0, 0, 0, 0, 0, 0],
            moves,
            pp,
            pp_ups: [0, 0, 0, 0],
            egg_steps_to_hatch: None,
            gender: self.gender.unwrap_or_else(|| pick_gender(&species_data.male_ratio)),
            nickname: None,
            met_at_date: SystemTime::now(),
            met_at_location: String::default(),
            met_at_level: level,
            pokerus: PokerusData::Unaffected,
            pokeball: None,

            // Battle stats
            status_condition: None,
            level,
            stats,
            current_hp: stats[0],
        }
    }
}

/// Generates a fully randomized Pokémon, according to the constraints of its
/// species.
pub fn generate_pokemon(
    species_data: &PokemonSpeciesData,
    move_dex: &MoveDex,
    level: usize,
) -> Pokemon {
    PokemonBuilder::default().build(&species_data, &move_dex, level)
}

pub fn pick_nature() -> Nature {
    let mut rng = thread_rng();
    let index = Uniform::new(0, Nature::count()).sample(&mut rng);

    Nature::by_index(index).unwrap()
}

pub fn pick_ivs() -> [usize; 6] {
    let mut rng = thread_rng();
    let random_iv = Uniform::new(0usize, 32);

    [
        random_iv.sample(&mut rng),
        random_iv.sample(&mut rng),
        random_iv.sample(&mut rng),
        random_iv.sample(&mut rng),
        random_iv.sample(&mut rng),
        random_iv.sample(&mut rng),
    ]
}

pub fn pick_moves(
    move_table: &Vec<(LearningCondition, String)>,
    level: usize,
) -> [Option<String>; MOVE_LIMIT] {
    let mut move_list = Vec::with_capacity(MOVE_LIMIT);

    move_table
        .iter()
        .rev()
        .filter(|(condition, _)| match condition {
            LearningCondition::Level(required_level) => *required_level <= level,
            _ => false,
        })
        .take(MOVE_LIMIT)
        .for_each(|(_, move_id)| move_list.push(move_id.clone()));

    let mut result: [Option<String>; MOVE_LIMIT] = Default::default();
    for (i, move_id) in move_list.into_iter().rev().enumerate() {
        result[i] = Some(move_id);
    }

    result
}

pub fn pick_stats(
    base_stats: &[usize; 6],
    evs: &[usize; 6],
    ivs: &[usize; 6],
    nature: Nature,
    level: usize,
) -> [usize; 6] {
    let mut result = [0; 6];

    for i in 0..6 {
        result[i] = ((2 * base_stats[i] + ivs[i] + (evs[i] / 4)) * level) / 100 + 5;
    }

    result[0] += level + 5;

    let nature = nature as usize;
    let increasing_stat = nature / 5 + 1;
    let decreasing_stat = nature % 5 + 1;

    if increasing_stat != decreasing_stat {
        result[increasing_stat] = (result[increasing_stat] as f32 * 1.1) as usize;
        result[decreasing_stat] = (result[decreasing_stat] as f32 * 0.9) as usize;
    }

    result
}

pub fn pick_pps(dex: &MoveDex, moves: &[Option<String>; MOVE_LIMIT]) -> [usize; MOVE_LIMIT] {
    let mut result: [usize; MOVE_LIMIT] = Default::default();

    moves
        .iter()
        .map(|mov| match mov {
            Some(mov) => dex.get_move(mov).unwrap().pp,
            None => 0,
        })
        .enumerate()
        .for_each(|(i, pp)| result[i] = pp);

    result
}

pub fn pick_ability(abilities: &Vec<String>) -> String {
    abilities.choose(&mut thread_rng()).unwrap().clone()
}

pub fn pick_gender(male_ratio: &Option<f32>) -> Gender {
    match male_ratio {
        Some(ratio) => {
            let mut rng = thread_rng();
            let choice = Uniform::new(0, 1000).sample(&mut rng);

            if choice < (10. * ratio) as usize {
                Gender::Male
            } else {
                Gender::Female
            }
        },
        None => Gender::Genderless,
    }
}
