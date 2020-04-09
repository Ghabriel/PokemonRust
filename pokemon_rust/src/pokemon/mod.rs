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
    status_conditions::{get_status_condition_effect, StatusConditionEffect},
};

/// Type effectiveness table. Every number is doubled (e.g 0.5x effectiveness
/// is stored as 1) so that we don't need to store floats.
const TYPE_TABLE: [[u8; 18]; 18] = [
    [2, 2, 2, 2, 2, 1, 2, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    [4, 2, 1, 1, 2, 4, 1, 0, 4, 2, 2, 2, 2, 1, 4, 2, 4, 1],
    [2, 4, 2, 2, 2, 1, 4, 2, 1, 2, 2, 4, 1, 2, 2, 2, 2, 2],
    [2, 2, 2, 1, 1, 1, 2, 1, 0, 2, 2, 4, 2, 2, 2, 2, 2, 4],
    [2, 2, 0, 4, 2, 4, 1, 2, 4, 4, 2, 1, 4, 2, 2, 2, 2, 2],
    [2, 1, 4, 2, 1, 2, 4, 2, 1, 4, 2, 2, 2, 2, 4, 2, 2, 2],
    [2, 1, 1, 1, 2, 2, 2, 1, 1, 1, 2, 4, 2, 4, 2, 2, 4, 1],
    [0, 2, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 4, 2, 2, 1, 2],
    [2, 2, 2, 2, 2, 4, 2, 2, 1, 1, 1, 2, 1, 2, 4, 2, 2, 4],
    [2, 2, 2, 2, 2, 1, 4, 2, 4, 1, 1, 4, 2, 2, 4, 1, 2, 2],
    [2, 2, 2, 2, 4, 4, 2, 2, 2, 4, 1, 1, 2, 2, 2, 1, 2, 2],
    [2, 2, 1, 1, 4, 4, 1, 2, 1, 1, 4, 1, 2, 2, 2, 1, 2, 2],
    [2, 2, 4, 2, 0, 2, 2, 2, 2, 2, 4, 1, 1, 2, 2, 1, 2, 2],
    [2, 4, 2, 4, 2, 2, 2, 2, 1, 2, 2, 2, 2, 1, 2, 2, 0, 2],
    [2, 2, 4, 2, 4, 2, 2, 2, 1, 1, 1, 4, 2, 2, 1, 4, 2, 2],
    [2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 2, 2, 2, 2, 2, 4, 2, 0],
    [2, 1, 2, 2, 2, 2, 2, 4, 2, 2, 2, 2, 2, 4, 2, 2, 1, 1],
    [2, 4, 2, 1, 2, 2, 2, 2, 1, 1, 2, 2, 2, 2, 2, 4, 4, 2],
];

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
    pub species_id: String,
    pub nature: Nature,
    pub held_item: Option<String>,
    pub experience_points: usize,
    pub ability: String,
    pub evs: [usize; 6],
    pub natural_ivs: [usize; 6],
    pub obtained_ivs: [usize; 6],
    pub moves: [Option<String>; MOVE_LIMIT],
    pub pp: [usize; MOVE_LIMIT],
    pub pp_ups: [usize; MOVE_LIMIT],
    pub egg_steps_to_hatch: Option<usize>,
    pub gender: Gender,
    pub nickname: Option<String>,
    pub met_at_date: SystemTime,
    pub met_at_location: String,
    pub met_at_level: usize,
    pub pokerus: PokerusData,
    pub pokeball: Option<String>,
    // pub shiny: bool,

    // Battle stats
    pub status_condition: Option<StatusCondition>,
    pub level: usize,
    pub stats: [usize; 6],
    pub current_hp: usize,
}

pub fn get_pokemon_display_name<'a>(pokemon: &'a Pokemon, pokedex: &'a PokeDex) -> &'a str {
    if let Some(name) = &pokemon.nickname {
        name
    } else {
        let species_id = &pokemon.species_id;
        let species = pokedex.get_species(species_id).unwrap();

        &species.display_name
    }
}

#[allow(unused)]
pub struct PokemonSpeciesData {
    pub id: String,
    pub display_name: String,
    pub national_number: usize,
    pub types: Vec<PokemonType>,
    pub base_stats: [usize; 6],
    pub male_ratio: Option<f32>,
    pub growth_rate: GrowthRate,
    pub base_exp_yield: usize,
    pub ev_yield: [usize; 6],
    pub capture_rate: usize,
    // pub base_friendship: usize,
    pub abilities: Vec<String>,
    pub hidden_abilities: Vec<String>,
    pub move_table: Vec<(LearningCondition, String)>,
    pub egg_moves: Vec<String>,
    pub egg_groups: Vec<String>,
    pub egg_steps: usize,
    pub height: f32,
    pub weight: f32,
    pub color: String,
    pub shape: usize,
    pub habitat: String,
    pub kind: String,
    pub pokedex_description: String,
    pub evolution_data: Vec<EvolutionData>,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusCondition {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic { counter: usize },
    Sleep { remaining_turns: usize },
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SimpleStatusCondition {
    Burn,
    Freeze,
    Paralysis,
    Poison,
    Toxic,
    Sleep,
}

impl From<StatusCondition> for SimpleStatusCondition {
    fn from(condition: StatusCondition) -> SimpleStatusCondition {
        match condition {
            StatusCondition::Burn => SimpleStatusCondition::Burn,
            StatusCondition::Freeze => SimpleStatusCondition::Freeze,
            StatusCondition::Paralysis => SimpleStatusCondition::Paralysis,
            StatusCondition::Poison => SimpleStatusCondition::Poison,
            StatusCondition::Toxic { .. } => SimpleStatusCondition::Toxic,
            StatusCondition::Sleep { .. } => SimpleStatusCondition::Sleep,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PokerusData {
    Unaffected,
    Cured,
    HasPokerus {
        duration: usize,
        remaining_days: usize,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl PokemonType {
    /// Returns the type effectiveness between two types.
    pub fn get_effectiveness(attacking_type: PokemonType, defending_type: PokemonType) -> f32 {
        let attacking_type = attacking_type as usize;
        let defending_type = defending_type as usize;

        (TYPE_TABLE[attacking_type][defending_type] as f32) / 2.
    }
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
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
