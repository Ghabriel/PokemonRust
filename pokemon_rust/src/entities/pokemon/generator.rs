use crate::entities::pokemon::{Gender, Nature, Pokemon, PokemonSpeciesData, PokerusData};

use rand::{
    distributions::{Distribution, Uniform},
    thread_rng,
};

use std::time::SystemTime;

pub fn generate_pokemon(species_data: &PokemonSpeciesData, level: usize) -> Pokemon {
    let nature = pick_nature();
    let evs = [0, 0, 0, 0, 0, 0];
    let ivs = pick_ivs();
    let moves = pick_moves(&species_data, level);
    let pp = pick_pps(&moves);
    let stats = pick_stats(&species_data.base_stats, &evs, &ivs, nature, level);

    Pokemon {
        species_id: species_data.id.clone(),
        nature,
        held_item: None,
        experience_points: 0,
        ability: None, // TODO
        evs,
        natural_ivs: ivs,
        obtained_ivs: [0, 0, 0, 0, 0, 0],
        moves,
        pp,
        pp_ups: [0, 0, 0, 0],
        egg_steps_to_hatch: None,
        gender: pick_gender(&species_data.male_ratio),
        form: 0, // TODO
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

pub fn pick_moves(species_data: &PokemonSpeciesData, level: usize) -> [Option<String>; 4] {
    // TODO
    [None, None, None, None]
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

pub fn pick_pps(moves: &[Option<String>; 4]) -> [usize; 4] {
    // TODO
    [0, 0, 0, 0]
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
