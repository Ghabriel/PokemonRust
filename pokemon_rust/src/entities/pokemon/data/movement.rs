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

use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MOVEDEX: MoveDex = {
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
            critical_hit: false,
        });

        result.push(Move {
            id: "Gust".to_string(),
            display_name: "Gust".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Flying,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Bite".to_string(),
            display_name: "Bite".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Dark,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(60),
            power_modifier: None,
            accuracy: Some(100),
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            // TODO: 30% flinch
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "MachPunch".to_string(),
            display_name: "Mach Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fight,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            pp: 30,
            priority: 1,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Scratch".to_string(),
            display_name: "Scratch".to_string(),
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
            critical_hit: false,
        });

        result.push(Move {
            id: "Slash".to_string(),
            display_name: "Slash".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(70),
            power_modifier: None,
            accuracy: Some(100),
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
        });

        MoveDex::new(
            result
                .into_iter()
                .map(|data| (data.id.clone(), data))
                .collect::<HashMap<_, _>>()
        )
    };
}

pub fn get_all_moves() -> &'static MoveDex {
    &MOVEDEX
}
