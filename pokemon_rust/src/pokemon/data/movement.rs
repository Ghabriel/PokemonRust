use crate::pokemon::{
    movement::{
        ModifiedAccuracy,
        ModifiedUsageAttempt,
        Move,
        MoveCategory,
        MoveDex,
        MoveFlag,
        MovePower,
        MultiHit,
        SecondaryEffect,
        SimpleEffect,
        SimpleEffectTarget,
        TargetType,
    },
    PokemonType,
    Stat,
    StatusCondition,
};

use lazy_static::lazy_static;

use std::collections::{HashMap, HashSet};

macro_rules! flags {
    [$($value:expr),*] => {
        {
            let mut set = HashSet::new();

            $(set.insert($value);)*

            set
        }
    }
}

lazy_static! {
    static ref MOVEDEX: MoveDex = {
        let mut result = Vec::new();

        result.push(Move {
            id: "Acid".to_string(),
            display_name: "Acid".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Poison,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::SpecialDefense, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Agility".to_string(),
            display_name: "Agility".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Speed, 2)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "AuroraBeam".to_string(),
            display_name: "Aurora Beam".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ice,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Attack, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Barrier".to_string(),
            display_name: "Barrier".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Defense, 2)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "BoneClub".to_string(),
            display_name: "Bone Club".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ground,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Bubble".to_string(),
            display_name: "Bubble".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Speed, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "BubbleBeam".to_string(),
            display_name: "Bubble Beam".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Speed, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "CometPunch".to_string(),
            display_name: "Comet Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(18),
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: Some(MultiHit::Custom(|mut rng| {
                let value = rng.check_custom_multi_hit(1, 6);

                match value {
                    1 | 2 => 2,
                    3 | 4 => 3,
                    5 => 4,
                    6 => 5,
                    _ => unreachable!(),
                }
            })),
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "ConfuseRay".to_string(),
            display_name: "Confuse Ray".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ghost,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_flag(target, "confusion") {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::Confusion,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Confusion".to_string(),
            display_name: "Confusion".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(50),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::Confusion,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Constrict".to_string(),
            display_name: "Constrict".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(10),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Speed, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "CrabHammer".to_string(),
            display_name: "Crab Hammer".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(100),
            power_modifier: None,
            accuracy: Some(90),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
        });

        result.push(Move {
            id: "Cut".to_string(),
            display_name: "Cut".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(50),
            power_modifier: None,
            accuracy: Some(95),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "DizzyPunch".to_string(),
            display_name: "Dizzy Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(70),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 20,
                effect: SimpleEffect::Confusion,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "DoubleKick".to_string(),
            display_name: "Double Kick".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fight,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(30),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: Some(MultiHit::Uniform {
                min_hits: 2,
                max_hits: 2,
            }),
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "DoubleSlap".to_string(),
            display_name: "Double Slap".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(15),
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: Some(MultiHit::Custom(|mut rng| {
                let value = rng.check_custom_multi_hit(1, 6);

                match value {
                    1 | 2 => 2,
                    3 | 4 => 3,
                    5 => 4,
                    6 => 5,
                    _ => unreachable!(),
                }
            })),
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "DoubleTeam".to_string(),
            display_name: "Double Team".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Evasion, 1)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "DrillPeck".to_string(),
            display_name: "Drill Peck".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Flying,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "EggBomb".to_string(),
            display_name: "Egg Bomb".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(100),
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Ember".to_string(),
            display_name: "Ember".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fire,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Burn),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "FireBlast".to_string(),
            display_name: "Fire Blast".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fire,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(110),
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 5,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Burn),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "FirePunch".to_string(),
            display_name: "Fire Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fire,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(75),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Burn),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Fissure".to_string(),
            display_name: "Fissure".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ground,
            category: MoveCategory::Physical,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(30),
            accuracy_modifier: Some(|user, target, _mov| {
                if target.level > user.level {
                    return ModifiedAccuracy::Miss;
                }

                return ModifiedAccuracy::NewValue(user.level - target.level + 30);
            }),
            flags: flags![MoveFlag::OneHitKO],
            on_usage_attempt: None,
            pp: 5,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Flamethrower".to_string(),
            display_name: "Flamethrower".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fire,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(90),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Burn),
            }),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            id: "Guillotine".to_string(),
            display_name: "Guillotine".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(30),
            accuracy_modifier: Some(|user, target, _mov| {
                if target.level > user.level {
                    return ModifiedAccuracy::Miss;
                }

                return ModifiedAccuracy::NewValue(user.level - target.level + 30);
            }),
            flags: flags![MoveFlag::OneHitKO],
            on_usage_attempt: None,
            pp: 5,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            id: "Headbutt".to_string(),
            display_name: "Headbutt".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(70),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "HornAttack".to_string(),
            display_name: "Horn Attack".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "HornDrill".to_string(),
            display_name: "Horn Drill".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(30),
            accuracy_modifier: Some(|user, target, _mov| {
                if target.level > user.level {
                    return ModifiedAccuracy::Miss;
                }

                return ModifiedAccuracy::NewValue(user.level - target.level + 30);
            }),
            flags: flags![MoveFlag::OneHitKO],
            on_usage_attempt: None,
            pp: 5,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "HydroPump".to_string(),
            display_name: "Hydro Pump".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(110),
            power_modifier: None,
            accuracy: Some(80),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 5,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "HyperFang".to_string(),
            display_name: "Hyper Fang".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(90),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Hypnosis".to_string(),
            display_name: "Hypnosis".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(60),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Sleep {
                    // TODO: randomize duration
                    remaining_turns: 1,
                }),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "IceBeam".to_string(),
            display_name: "Ice Beam".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ice,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(90),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Freeze),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "IcePunch".to_string(),
            display_name: "Ice Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ice,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(75),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Freeze),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
        });

        result.push(Move {
            id: "Leer".to_string(),
            display_name: "Leer".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            id: "Lick".to_string(),
            display_name: "Lick".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ghost,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(30),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 1,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Meditate".to_string(),
            display_name: "Meditate".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 40,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Attack, 1)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "MegaKick".to_string(),
            display_name: "Mega Kick".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(120),
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 5,
            priority: 0,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Peck".to_string(),
            display_name: "Peck".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Flying,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(35),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "PoisonPowder".to_string(),
            display_name: "Poison Powder".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Poison,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Poison),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "PoisonSting".to_string(),
            display_name: "Poison Sting".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Poison,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(15),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::StatusCondition(StatusCondition::Poison),
            }),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Psychic".to_string(),
            display_name: "Psychic".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(90),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::SpecialDefense, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Psybeam".to_string(),
            display_name: "Psybeam".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Psychic,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::Confusion,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "QuickAttack".to_string(),
            display_name: "Quick Attack".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 1,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "RazorLeaf".to_string(),
            display_name: "Razor Leaf".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Grass,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(55),
            power_modifier: None,
            accuracy: Some(95),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
        });

        result.push(Move {
            id: "RockSlide".to_string(),
            display_name: "Rock Slide".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Rock,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(75),
            power_modifier: None,
            accuracy: Some(90),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 10,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "RockThrow".to_string(),
            display_name: "Rock Throw".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Rock,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(50),
            power_modifier: None,
            accuracy: Some(90),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "RollingKick".to_string(),
            display_name: "Rolling Kick".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Fight,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(60),
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            // TODO: Sand Attack should still affect flying-types and Pokémon
            // with Levitate
            id: "SandAttack".to_string(),
            display_name: "Sand Attack".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Ground,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Accuracy, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Screech".to_string(),
            display_name: "Screech".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(85),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 40,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Defense, -2)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Sing".to_string(),
            display_name: "Sing".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(55),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Sleep {
                    // TODO: randomize duration
                    remaining_turns: 1,
                }),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Slam".to_string(),
            display_name: "Slam".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: true,
        });

        result.push(Move {
            id: "SleepPowder".to_string(),
            display_name: "Sleep Powder".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Grass,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Sleep {
                    // TODO: randomize duration
                    remaining_turns: 1,
                }),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Sludge".to_string(),
            display_name: "Sludge".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Poison,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(65),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 30,
                effect: SimpleEffect::StatusCondition(StatusCondition::Poison),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Smog".to_string(),
            display_name: "Smog".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Poison,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(30),
            power_modifier: None,
            accuracy: Some(70),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 40,
                effect: SimpleEffect::StatusCondition(StatusCondition::Poison),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Smokescreen".to_string(),
            display_name: "Smokescreen".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Accuracy, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Strength".to_string(),
            display_name: "Strength".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "StringShot".to_string(),
            display_name: "String Shot".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Bug,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(95),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 40,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Speed, -1)],
                    target: SimpleEffectTarget::MoveTarget,
                }
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "StunSpore".to_string(),
            display_name: "Stun Spore".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Grass,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(75),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Supersonic".to_string(),
            display_name: "Supersonic".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(55),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_flag(target, "confusion") {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::Confusion,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "Swift".to_string(),
            display_name: "Swift".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(60),
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::AllAdjacentFoes,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "SwordsDance".to_string(),
            display_name: "Swords Dance".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Normal,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 20,
            priority: 0,
            target_type: TargetType::User,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatChange {
                    changes: vec![(Stat::Attack, 2)],
                    target: SimpleEffectTarget::MoveUser,
                }
            }),
            critical_hit: false,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
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
            id: "Thunderbolt".to_string(),
            display_name: "Thunderbolt".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Electric,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(90),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "ThunderPunch".to_string(),
            display_name: "Thunder Punch".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Electric,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(75),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "ThunderShock".to_string(),
            display_name: "Thunder Shock".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Electric,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 10,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "ThunderWave".to_string(),
            display_name: "Thunder Wave".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Electric,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: Some(90),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: Some(|backend, _user, target, _mov| {
                if backend.has_non_volatile_status_condition(target) {
                    return ModifiedUsageAttempt::Fail;
                }

                ModifiedUsageAttempt::Continue
            }),
            pp: 20,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 100,
                effect: SimpleEffect::StatusCondition(StatusCondition::Paralysis),
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "VineWhip".to_string(),
            display_name: "Vine Whip".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Grass,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(45),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 30,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Waterfall".to_string(),
            display_name: "Waterfall".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Physical,
            base_power: MovePower::Constant(80),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 15,
            priority: 0,
            target_type: TargetType::SingleAdjacentTarget,
            multi_hit: None,
            secondary_effect: Some(SecondaryEffect {
                chance: 20,
                effect: SimpleEffect::Flinch,
            }),
            critical_hit: false,
        });

        result.push(Move {
            id: "WaterGun".to_string(),
            display_name: "Water Gun".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Special,
            base_power: MovePower::Constant(40),
            power_modifier: None,
            accuracy: Some(100),
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 25,
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
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 35,
            priority: 0,
            target_type: TargetType::SingleTarget,
            multi_hit: None,
            secondary_effect: None,
            critical_hit: false,
        });

        result.push(Move {
            id: "Withdraw".to_string(),
            display_name: "Withdraw".to_string(),
            description: "".to_string(), // TODO
            move_type: PokemonType::Water,
            category: MoveCategory::Status,
            base_power: MovePower::Special,
            power_modifier: None,
            accuracy: None,
            accuracy_modifier: None,
            flags: HashSet::new(),
            on_usage_attempt: None,
            pp: 40,
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
