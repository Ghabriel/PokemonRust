mod generator;

use amethyst::ecs::World;

use crate::constants::MOVE_LIMIT;

use std::time::SystemTime;

pub struct Pokemon {
    species_id: String,
    nature: Nature,
    held_item: Option<String>,
    experience_points: usize,
    ability: String,
    evs: [usize; 6],
    natural_ivs: [usize; 6],
    obtained_ivs: [usize; 6],
    moves: [Option<String>; MOVE_LIMIT],
    pp: [usize; MOVE_LIMIT],
    pp_ups: [usize; MOVE_LIMIT],
    egg_steps_to_hatch: Option<usize>,
    gender: Gender,
    nickname: Option<String>,
    met_at_date: SystemTime,
    met_at_location: String,
    met_at_level: usize,
    pokerus: PokerusData,
    pokeball: Option<String>,
    // shiny: bool,

    // Battle stats
    status_condition: Option<StatusCondition>,
    level: usize,
    stats: [usize; 6],
    current_hp: usize,
}

pub struct PokemonSpeciesData {
    id: String,
    display_name: String,
    national_number: usize,
    types: Vec<PokemonType>,
    base_stats: [usize; 6],
    male_ratio: Option<f32>,
    growth_rate: GrowthRate,
    base_exp_yield: usize,
    ev_yield: [usize; 6],
    capture_rate: usize,
    // base_friendship: usize,
    abilities: Vec<String>,
    hidden_abilities: Vec<String>,
    move_table: Vec<(usize, String)>,
    egg_moves: Vec<String>,
    egg_groups: Vec<String>,
    egg_steps: usize,
    height: f32,
    weight: f32,
    color: String,
    shape: usize,
    habitat: String,
    kind: String,
    pokedex_description: String,
    evolution_data: Vec<EvolutionData>,
}

#[derive(Clone, Copy)]
pub enum Nature {
    Hardy,
    Lonely,
    Adamant,
    Naughty,
    Brave,
    Bold,
    Docile,
    Impish,
    Lax,
    Relaxed,
    Modest,
    Mild,
    Bashful,
    Rash,
    Quiet,
    Calm,
    Gentle,
    Careful,
    Quirky,
    Sassy,
    Timid,
    Hasty,
    Jolly,
    Naive,
    Serious,
}

impl Nature {
    pub fn count() -> usize {
        25
    }

    pub fn by_index(index: usize) -> Option<Nature> {
        match index {
            0 => Some(Self::Hardy),
            1 => Some(Self::Lonely),
            2 => Some(Self::Adamant),
            3 => Some(Self::Naughty),
            4 => Some(Self::Brave),
            5 => Some(Self::Bold),
            6 => Some(Self::Docile),
            7 => Some(Self::Impish),
            8 => Some(Self::Lax),
            9 => Some(Self::Relaxed),
            10 => Some(Self::Modest),
            11 => Some(Self::Mild),
            12 => Some(Self::Bashful),
            13 => Some(Self::Rash),
            14 => Some(Self::Quiet),
            15 => Some(Self::Calm),
            16 => Some(Self::Gentle),
            17 => Some(Self::Careful),
            18 => Some(Self::Quirky),
            19 => Some(Self::Sassy),
            20 => Some(Self::Timid),
            21 => Some(Self::Hasty),
            22 => Some(Self::Jolly),
            23 => Some(Self::Naive),
            24 => Some(Self::Serious),
            _ => None,
        }
    }
}

pub enum Gender {
    Male,
    Female,
    Genderless,
}

pub enum StatusCondition {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic { counter: usize },
    Sleep { remaining_turns: usize },
}

pub enum PokerusData {
    Unaffected,
    Cured,
    HasPokerus { duration: usize, remaining_days: usize },
}

pub enum PokemonType {
    Normal,
    Fight,
    Flying,
    Poison,
    Ground,
    Rock,
    Bug,
    Ghost,
    Steel,
    Fire,
    Water,
    Grass,
    Electric,
    Psychic,
    Ice,
    Dragon,
    Dark,
    Fairy,
}

pub enum GrowthRate {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

pub struct EvolutionData {
    pokemon: EvolutionTarget,
    triggering_event: EvolutionEvent,
    conditions: Vec<EvolutionCondition>,
}

pub enum EvolutionTarget {
    /// Evolution to the same Pokémon regardless of the circumstances.
    /// Applies to almost all Pokémon. If the Pokémon has multiple forms,
    /// its evolution maintains the same form.
    Static(String),
    /// Evolution that depends on the circumstances. Examples of this include
    /// Burmy -> Wormadam and Toxel -> Toxtricity.
    Dynamic(fn(&Pokemon, &World) -> Pokemon),
}

/// The event that triggers an evolution. In the official games, Pokémon can
/// also evolve by being traded, but we won't have that option.
pub enum EvolutionEvent {
    LevelUp(usize),
    EvolutionStone(String),
}

pub enum EvolutionCondition {
    // HighFriendship(usize),
    /// Evolution by holding an item, e.g Clamperl
    HoldingItem(String),
    /// Evolution by time of day, e.g Umbreon
    TimeOfDay(TimeOfDay),
    // KnowingMove(String),
    /// Evolution at a certain location, e.g Magneton
    Location(String),
    // HavingPokemonInParty(String),
}

pub enum TimeOfDay {
    /// 04:00 - 09:59
    Morning,
    /// 10:00 - 17:59
    Day,
    /// 18:00 - 03:59
    Night,
}

pub enum Stat {
    HP,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasion,
}
