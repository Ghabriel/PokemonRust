use crate::entities::pokemon::{
    GrowthRate,
    LearningCondition,
    Pokemon,
    PokemonSpeciesData,
    PokemonType,
};

use std::collections::HashMap;

pub fn list<T, U>(source: Vec<T>) -> Vec<U>
where
    T: Into<U>,
{
    source
        .into_iter()
        .map(Into::into)
        .collect()
}

pub fn tuple_list<T, U, V, W>(source: Vec<(T, U)>) -> Vec<(V, W)>
where
    T: Into<V>,
    U: Into<W>,
{
    source
        .into_iter()
        .map(|(t, u)| (t.into(), u.into()))
        .collect()
}

pub fn get_pokemon_species_data() -> HashMap<String, PokemonSpeciesData> {
    let mut result = Vec::new();

    result.push(PokemonSpeciesData {
        id: "Pidgey".to_string(),
        display_name: "Pidgey".to_string(),
        national_number: 16,
        types: vec![PokemonType::Normal, PokemonType::Flying],
        base_stats: [40, 45, 40, 35, 35, 56],
        male_ratio: Some(50.),
        growth_rate: GrowthRate::MediumSlow,
        base_exp_yield: 50,
        ev_yield: [0, 0, 0, 0, 0, 1],
        capture_rate: 255,
        abilities: list(vec!["KeenEye", "TangledFeet"]),
        hidden_abilities: list(vec!["BigPecks"]),
        move_table: tuple_list(vec![
            (LearningCondition::Level(1), "Tackle"),
            (LearningCondition::Level(5), "SandAttack"),
            (LearningCondition::Level(9), "Gust"),
            (LearningCondition::Level(13), "QuickAttack"),
            (LearningCondition::Level(17), "Whirlwind"),
            (LearningCondition::Level(21), "Twister"),
            (LearningCondition::Level(25), "FeatherDance"),
            (LearningCondition::Level(29), "Agility"),
            (LearningCondition::Level(33), "WingAttack"),
            (LearningCondition::Level(37), "Roost"),
            (LearningCondition::Level(41), "TailWind"),
            (LearningCondition::Level(45), "MirrorMove"),
            (LearningCondition::Level(49), "AirSlash"),
            (LearningCondition::Level(53), "Hurricane"),
        ]),
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
    });

    result
        .into_iter()
        .map(|data| (data.id.clone(), data))
        .collect()
}
