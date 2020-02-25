use crate::entities::pokemon::{
    GrowthRate,
    LearningCondition,
    PokeDex,
    PokemonSpeciesData,
    PokemonType,
};

use lazy_static::lazy_static;

use std::collections::HashMap;

macro_rules! species {
    (
        id: $id:literal,
        display_name: $display_name:literal,
        national_number: $national_number:literal,
        types: [$( $types:expr ),*],
        base_stats: $base_stats:expr,
        male_ratio: $male_ratio:expr,
        growth_rate: $growth_rate:expr,
        base_exp_yield: $base_exp_yield:literal,
        ev_yield: $ev_yield:expr,
        capture_rate: $capture_rate:literal,
        abilities: [$( $abilities:literal ),*],
        hidden_abilities: [$( $hidden_abilities:literal ),*],
        move_table: [
            $( $level:literal: $movement:literal ),* $(,)?
        ],
    ) => {
        PokemonSpeciesData {
            id: $id.to_string(),
            display_name: $display_name.to_string(),
            national_number: $national_number,
            types: vec![$( $types ),*],
            base_stats: $base_stats,
            male_ratio: $male_ratio,
            growth_rate: $growth_rate,
            base_exp_yield: $base_exp_yield,
            ev_yield: $ev_yield,
            capture_rate: $capture_rate,
            abilities: vec![$( $abilities.into() ),*],
            hidden_abilities: vec![$( $hidden_abilities.into() ),*],
            move_table: vec![
                $((LearningCondition::Level($level), $movement.into())),*
            ],
            egg_moves: Vec::new(), // TODO
            egg_groups: Vec::new(), // TODO
            egg_steps: 0, // TODO
            height: 0., // TODO
            weight: 0., // TODO
            color: "".to_string(), // TODO
            shape: 0, // TODO
            habitat: "".to_string(), // TODO
            kind: "".to_string(), // TODO
            pokedex_description: "".to_string(), // TODO
            evolution_data: Vec::new(), // TODO
        }
    }
}

lazy_static! {
    static ref POKEDEX: PokeDex = {
        let mut result = Vec::new();

        result.push(species! {
            id: "Pidgey",
            display_name: "Pidgey",
            national_number: 16,
            types: [PokemonType::Normal, PokemonType::Flying],
            base_stats: [40, 45, 40, 35, 35, 56],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 50,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 255,
            abilities: ["KeenEye", "TangledFeet"],
            hidden_abilities: ["BigPecks"],
            move_table: [
                1: "Tackle",
                // 5: "SandAttack",
                // 9: "Gust",
                // 13: "QuickAttack",
                // 17: "Whirlwind",
                // 21: "Twister",
                // 25: "FeatherDance",
                // 29: "Agility",
                // 33: "WingAttack",
                // 37: "Roost",
                // 41: "TailWind",
                // 45: "MirrorMove",
                // 49: "AirSlash",
                // 53: "Hurricane",
            ],
        });

        result.push(species! {
            id: "Rattata",
            display_name: "Rattata",
            national_number: 19,
            types: [PokemonType::Normal],
            base_stats: [30, 56, 35, 25, 35, 72],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 51,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 255,
            abilities: ["RunAway", "Guts"],
            hidden_abilities: ["Hustle"],
            move_table: [
                1: "Tackle",
                // 1: "TailWhip",
                // 4: "QuickAttack",
                // 7: "FocusEnergy",
                // 10: "Bite",
                // 13: "Pursuit",
                // 16: "HyperFang",
                // 19: "SuckerPunch",
                // 22: "Crunch",
                // 25: "Assurance",
                // 28: "SuperFang",
                // 31: "DoubleEdge",
                // 34: "Endeavor",
            ],
        });

        PokeDex::new(
            result
                .into_iter()
                .map(|data| (data.id.clone(), data))
                .collect::<HashMap<_, _>>()
        )
    };
}

pub fn get_all_pokemon_species() -> &'static PokeDex {
    &POKEDEX
}
