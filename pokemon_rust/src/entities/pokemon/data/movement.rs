use crate::entities::pokemon::{
    movement::{
        Move,
        MoveCategory,
        MoveDex,
        MovePower,
        TargetType,
    },
    PokemonType,
};

use std::collections::HashMap;

pub fn get_all_moves() -> MoveDex {
    let mut result = Vec::new();

    result.push(Move {
        id: "Tackle".to_string(),
        display_name: "Tackle".to_string(),
        description: "".to_string(), // TODO
        move_type: PokemonType::Normal,
        category: MoveCategory::Physical,
        base_power: MovePower::Constant(40),
        power_modifier: None,
        accuracy: Some(100),
        pp: 35,
        priority: 0,
        target_type: TargetType::SingleAdjacentTarget,
        multi_hit: None,
        secondary_effect: None,
    });

    MoveDex::new(
        result
            .into_iter()
            .map(|data| (data.id.clone(), data))
            .collect::<HashMap<_, _>>()
    )
}
