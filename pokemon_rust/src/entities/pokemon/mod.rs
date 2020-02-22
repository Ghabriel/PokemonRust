mod data;
pub mod generator;
pub mod movement;

use amethyst::ecs::World;

use crate::constants::MOVE_LIMIT;

use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
    time::SystemTime,
};

pub use self::data::{
    movement::get_all_moves,
    pokemon::get_all_pokemon_species,
};

pub struct PokeDex {
    data: HashMap<String, PokemonSpeciesData>,
}

impl PokeDex {
    pub fn new(data: HashMap<String, PokemonSpeciesData>) -> PokeDex {
        PokeDex { data }
    }

    pub fn get_species(&self, id: &str) -> Option<&PokemonSpeciesData> {
        self.data.get(id)
    }
}

#[derive(Clone, Debug)]
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

#[allow(unused)]
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
    move_table: Vec<(LearningCondition, String)>,
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

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Debug)]
pub enum Gender {
    Male,
    Female,
    Genderless,
}

#[derive(Clone, Debug)]
pub enum StatusCondition {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic { counter: usize },
    Sleep { remaining_turns: usize },
}

#[derive(Clone, Debug)]
pub enum PokerusData {
    Unaffected,
    Cured,
    HasPokerus { duration: usize, remaining_days: usize },
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum GrowthRate {
    Erratic,
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
    Fluctuating,
}

#[derive(Clone, Debug)]
pub enum LearningCondition {
    Level(usize),
    Evolution,
}

/// Contains data about a possible Pokémon evolution.
#[derive(Clone, Debug)]
pub struct EvolutionData {
    /// The target of this evolution, i.e to which Pokémon this one evolves to.
    pokemon: EvolutionTarget,
    /// The event that triggers the evolution, as soon as the right conditions
    /// are fulfilled.
    triggering_event: EvolutionEvent,
    /// The conditions that must hold for the evolution to occur.
    conditions: Vec<EvolutionCondition>,
}

/// Represents a Pokémon that another Pokémon can evolve to.
#[derive(Clone)]
pub enum EvolutionTarget {
    /// Evolution to the same Pokémon regardless of the circumstances.
    /// Applies to almost all Pokémon. If the Pokémon has multiple forms,
    /// its evolution maintains the same form.
    Static(String),
    /// Evolution that depends on the circumstances. Examples of this include
    /// Tyrogue -> {Hitmonlee, Hitmonchan, Hitmontop}, Burmy -> Wormadam and
    /// Toxel -> Toxtricity.
    Dynamic(fn(&Pokemon, &World) -> Pokemon),
}

impl Debug for EvolutionTarget {
    fn fmt(&self, formatter: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Static(target) => write!(formatter, "Static({})", target),
            Self::Dynamic(_) => write!(formatter, "Dynamic"),
        }
    }
}

/// The event that triggers an evolution. In the official games, Pokémon can
/// also evolve by being traded, but we won't have that option. There are a few
/// other possibilities that have been changed:
///  * Galarian Farfetch'd evolves into Sirfetch'd by leveling up while holding
///    a Leek;
///  * Galarian Yamask evolves into Runerigus by leveling up in a certain
///    location;
///  * Milcery evolves into Alcremie by leveling up while holding a certain
///    item (the item determines the Alcremie's flavor).
#[derive(Clone, Debug)]
pub enum EvolutionEvent {
    LevelUp(usize),
    EvolutionStone(String),
}

#[derive(Clone, Debug)]
pub enum EvolutionCondition {
    // HighFriendship(usize),
    /// Evolution by holding an item, e.g Clamperl
    HoldingItem(String),
    /// Evolution by time of day, e.g Eevee -> Umbreon
    TimeOfDay(TimeOfDay),
    // Evolution by knowing a move, e.g Steenee -> Tsareena
    KnowingMove(String),
    /// Evolution at a certain location, e.g Magneton -> Magnezone
    Location(String),
    /// Evolution by having a certain gender, e.g Kirlia -> Gallade
    Gender(Gender),
    /// Evolution by having a certain Pokémon in the party, e.g Mantyke -> Mantine
    HavingPokemonInParty(String),
    /// Evolution by having a Pokémon of a certain type in the party, e.g
    /// Pancham -> Pangoro
    HavingTypeInParty(PokemonType),
    /// Evolution by the weather of the overworld, e.g Sliggoo -> Goodra
    Weather(String),
}

#[derive(Clone, Debug)]
pub enum TimeOfDay {
    /// 04:00 - 09:59
    Morning,
    /// 10:00 - 17:59
    Day,
    /// 18:00 - 03:59
    Night,
}

#[derive(Clone, Debug)]
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
