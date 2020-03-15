use crate::entities::pokemon::{
    movement::{
        Move,
        MoveCategory,
        MoveDex,
        MovePower,
        SecondaryEffect,
        SimpleEffect,
        SimpleEffectTarget,
        TargetType,
    },
    PokemonType,
    Stat,
};

use lazy_static::lazy_static;

use std::collections::HashMap;

lazy_static! {
    static ref MOVEDEX: MoveDex = {
        let mut result = Vec::new();

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
            id: "Growl".to_string(),
            display_name: "Growl".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            pp: 40,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Attack, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
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
            id: "Harden".to_string(),
            display_name: "Harden".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Defense, 1)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "KarateChop".to_string(),
            display_name: "Karate Chop".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fight,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(50),
            power_modifier: None,
            accuracy: Some(100),
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
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
            id: "MegaPunch".to_string(),
            display_name: "Mega Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(85),
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Pound".to_string(),
            display_name: "Pound".to_string(),
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
            id: "TailWhip".to_string(),
            display_name: "Tail Whip".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            pp: 30,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Defense, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "ViseGrip".to_string(),
            display_name: "Vise Grip".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(55),
            power_modifier: None,
            accuracy: Some(100),
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "WingAttack".to_string(),
            display_name: "Wing Attack".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Flying,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(60),
            power_modifier: None,
            accuracy: Some(100),
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
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
