use amethyst::ecs::World;

use std::time::SystemTime;

pub struct Pokemon {
    species: String,
    nature: Nature,
    held_item: Option<String>,
    experience_points: usize,
    ability: Option<String>,
    evs: [usize; 6],
    natural_ivs: [usize; 6],
    obtained_ivs: [usize; 6],
    moves: [Option<String>; 4],
    pp: [usize; 4],
    pp_ups: [usize; 4],
    egg_steps_to_hatch: Option<usize>,
    gender: Gender,
    form: usize,
    nickname: Option<String>,
    met_at_date: SystemTime,
    met_at_location: String,
    met_at_level: usize,
    pokerus: PokerusData,
    pokeball: String,

    // Battle stats
    status_condition: Option<StatusCondition>,
    level: usize,
    stats: [usize; 6],
    current_hp: usize,
}

pub struct PokemonSpeciesData {
    display_name: String,
    national_number: usize,
    types: Vec<PokemonType>,
    base_stats: [usize; 6],
    male_ratio: f32,
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
