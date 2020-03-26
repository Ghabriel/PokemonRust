use crate::pokemon::{GrowthRate, LearningCondition, PokeDex, PokemonSpeciesData, PokemonType};

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
            id: "Bulbasaur",
            display_name: "Bulbasaur",
            national_number: 1,
            types: [PokemonType::Grass, PokemonType::Poison],
            base_stats: [45, 49, 49, 65, 65, 45],
            male_ratio: Some(87.5),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 64,
            ev_yield: [0, 0, 0, 1, 0, 0],
            capture_rate: 45,
            abilities: ["Overgrow"],
            hidden_abilities: ["Chlorophyll"],
            move_table: [
                1: "Tackle",
                1: "Growl",
                3: "VineWhip",
                // 6: "Growth",
                // 9: "LeechSeed",
                // 12: "RazorLeaf",
                // 15: "PoisonPowder",
                // 15: "SleepPowder",
                // 18: "SeedBomb",
                // 21: "TakeDown",
                // 24: "SweetScent",
                // 27: "Synthesis",
                // 30: "WorrySeed",
                // 33: "DoubleEdge",
                // 36: "SolarBeam",
            ],
        });

        result.push(species! {
            id: "Charmander",
            display_name: "Charmander",
            national_number: 4,
            types: [PokemonType::Fire],
            base_stats: [39, 52, 43, 60, 50, 65],
            male_ratio: Some(87.5),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 62,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 45,
            abilities: ["Blaze"],
            hidden_abilities: ["SolarPower"],
            move_table: [
                1: "Scratch",
                1: "Growl",
                // 4: "Ember",
                // 8: "Smokescreen",
                // 12: "DragonBreath",
                // 17: "FireFang",
                20: "Slash",
                // 24: "Flamethrower",
                // 28: "ScaryFace",
                // 32: "FireSpin",
                // 36: "Inferno",
                // 40: "FlareBlitz",
            ],
        });

        result.push(species! {
            id: "Squirtle",
            display_name: "Squirtle",
            national_number: 7,
            types: [PokemonType::Water],
            base_stats: [44, 48, 65, 50, 64, 43],
            male_ratio: Some(87.5),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 63,
            ev_yield: [0, 0, 1, 0, 0, 0],
            capture_rate: 45,
            abilities: ["Torrent"],
            hidden_abilities: ["RainDish"],
            move_table: [
                1: "Tackle",
                1: "TailWhip",
                3: "WaterGun",
                // 6: "Withdraw",
                // 9: "RapidSpin",
                // 12: "Bite",
                // 15: "WaterPulse",
                // 18: "Protect",
                // 21: "RainDance",
                // 24: "AquaTail",
                // 27: "ShellSmash",
                // 30: "IronDefense",
                33: "HydroPump",
                // 36: "SkullBash",
            ],
        });

        result.push(species! {
            id: "Caterpie",
            display_name: "Caterpie",
            national_number: 10,
            types: [PokemonType::Bug],
            base_stats: [45, 30, 35, 20, 20, 45],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 39,
            ev_yield: [1, 0, 0, 0, 0, 0],
            capture_rate: 255,
            abilities: ["ShieldDust"],
            hidden_abilities: ["RunAway"],
            move_table: [
                1: "Tackle",
                1: "StringShot",
                9: "BugBite",
            ],
        });

        result.push(species! {
            id: "Metapod",
            display_name: "Metapod",
            national_number: 11,
            types: [PokemonType::Bug],
            base_stats: [50, 20, 55, 25, 25, 30],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 72,
            ev_yield: [0, 0, 2, 0, 0, 0],
            capture_rate: 120,
            abilities: ["ShedSkin"],
            hidden_abilities: [],
            move_table: [
                1: "Harden",
                // TODO: learns when evolving
                // 1: "Harden",
            ],
        });

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
                9: "Gust",
                13: "QuickAttack",
                // 17: "Whirlwind",
                // 21: "Twister",
                // 25: "FeatherDance",
                // 29: "Agility",
                33: "WingAttack",
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
                3: "TailWhip",
                6: "QuickAttack",
                // 9: "FocusEnergy",
                12: "Bite",
                // 15: "SuperFang",
                // 18 "Crunch",
                // 21: "HyperFang",
                // 24: "SuckerPunch",
                // 27: "Double-Edge",
            ],
        });

        result.push(species! {
            id: "Clefairy",
            display_name: "Clefairy",
            national_number: 35,
            types: [PokemonType::Fairy],
            base_stats: [70, 45, 48, 60, 65, 35],
            male_ratio: Some(25.),
            growth_rate: GrowthRate::Fast,
            base_exp_yield: 113,
            ev_yield: [2, 0, 0, 0, 0, 0],
            capture_rate: 150,
            abilities: ["CuteCharm", "MagicGuard"],
            hidden_abilities: ["FriendGuard"],
            move_table: [
                // 1: "Sing",
                // 1: "SweetKiss",
                // 1: "DisarmingVoice",
                // 1: "Encore",
                // 1: "Charm",
                // 1: "Splash",
                1: "Pound",
                // 1: "Copycat",
                1: "Growl",
                // 1: "DefenseCurl",
                // 4: "StoredPower",
                // 8: "Minimize",
                10: "DoubleSlap",
                // 12: "AfterYou",
                // 16: "LifeDew",
                // 20: "Metronome",
                // 24: "Moonlight",
                // 28: "Gravity",
                // 32: "MeteorMash",
                // 36: "FollowMe",
                // 40: "CosmicPower",
                // 44: "MoonBlast",
                // 48: "HealingWish",
            ],
        });

        result.push(species! {
            id: "Machop",
            display_name: "Machop",
            national_number: 66,
            types: [PokemonType::Fight],
            base_stats: [70, 45, 48, 60, 65, 35],
            male_ratio: Some(75.),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 61,
            ev_yield: [0, 1, 0, 0, 0, 0],
            capture_rate: 180,
            abilities: ["Guts", "NoGuard"],
            hidden_abilities: ["Steadfast"],
            move_table: [
                // 1: "LowKick",
                // 1: "Leer",
                // 3: "FocusEnergy",
                7: "KarateChop",
                // 9: "Foresight",
                // 13: "LowSweep",
                // 15: "SeismicToss",
                // 19: "Revenge",
                // 21: "KnockOff",
                // 25: "VitalThrow",
                // 27: "Wake-UpSlap",
                // 31: "DualChop",
                // 33: "Submission",
                // 37: "BulkUp",
                // 39: "CrossChop",
                // 43: "ScaryFace",
                // 45: "DynamicPunch",
            ],
        });

        result.push(species! {
            id: "Farfetchd",
            display_name: "Farfetch'd",
            national_number: 83,
            types: [PokemonType::Normal, PokemonType::Flying],
            base_stats: [52, 90, 55, 58, 62, 60],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 132,
            ev_yield: [0, 1, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["KeenEye", "InnerFocus"],
            hidden_abilities: ["Defiant"],
            move_table: [
                1: "Peck",
                // 1: "SandAttack",
                // 5: "Leer",
                // 10: "FuryCutter",
                15: "Cut",
                // 20: "AerialAce",
                // 25: "AirCutter",
                // 30: "KnockOff",
                // 35: "FalseSwipe",
                // 40: "Slash",
                // 45: "SwordsDance",
                // 50: "AirSlash",
                // 55: "LeafBlade",
                // 60: "Agility",
                // 65: "BraveBird",
            ],
        });

        result.push(species! {
            id: "Krabby",
            display_name: "Krabby",
            national_number: 98,
            types: [PokemonType::Water],
            base_stats: [30, 105, 90, 25, 25, 50],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 65,
            ev_yield: [0, 1, 0, 0, 0, 0],
            capture_rate: 225,
            abilities: ["HyperCutter", "ShellArmor"],
            hidden_abilities: ["SheerForce"],
            move_table: [
                1: "ViseGrip",
                1: "WaterGun",
                // 1: "Leer",
                4: "Harden",
                // 8: "MetalClaw",
                // 12: "MudShot",
                // 16: "Protect",
                // 20: "BubbleBeam",
                // 24: "Stomp",
                // 29: "Flail",
                // 32: "RazorShell",
                36: "Slam",
                // 40: "SwordsDance",
                44: "CrabHammer",
                // 48: "Guillotine",
            ],
        });

        result.push(species! {
            id: "Hitmonlee",
            display_name: "Hitmonlee",
            national_number: 106,
            types: [PokemonType::Fight],
            base_stats: [50, 120, 53, 35, 110, 87],
            male_ratio: Some(100.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 159,
            ev_yield: [0, 2, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["Limber", "Reckless"],
            hidden_abilities: ["Unburden"],
            move_table: [
                // 1: "BrickBreak",
                // 1: "Feint",
                // 1: "LowSweep",
                1: "Tackle",
                // 1: "HelpingHand",
                // 1: "FakeOut",
                // 1: "FocusEnergy",
                // TODO: learns when evolving
                // 1: "BrickBreak",
                // 4: "DoubleKick",
                // 8: "LowKick",
                // 12: "Endure",
                // 16: "Revenge",
                // 21: "WideGuard",
                // 24: "BlazeKick",
                // 28: "MindReader",
                32: "MegaKick",
                // 36: "CloseCombat",
                // 40: "Reversal",
                // 44: "HighJumpKick",
            ],
        });

        result.push(species! {
            id: "Hitmonchan",
            display_name: "Hitmonchan",
            national_number: 107,
            types: [PokemonType::Fight],
            base_stats: [50, 105, 79, 35, 110, 76],
            male_ratio: Some(100.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 159,
            ev_yield: [0, 0, 0, 0, 2, 0],
            capture_rate: 45,
            abilities: ["KeenEye", "IronFist"],
            hidden_abilities: ["InnerFocus"],
            move_table: [
                // 1: "DrainPunch",
                // 1: "Feint",
                // 1: "VacuumWave",
                // 1: "BulletPunch",
                1: "Tackle",
                // 1: "HelpingHand",
                // 1: "FakeOut",
                // 1: "FocusEnergy",
                // // TODO: learns when evolving
                // 1: "DrainPunch",
                4: "MachPunch",
                // 8: "Power-UpPunch",
                // 12: "Detect",
                // 16: "Revenge",
                // 21: "QuickGuard",
                // 24: "ThunderPunch",
                // 24: "IcePunch",
                // 24: "FirePunch",
                // 28: "Agility",
                32: "MegaPunch",
                // 36: "CloseCombat",
                // 40: "Counter",
                // 44: "FocusPunch",
            ],
        });

        result.push(species! {
            id: "Rhyhorn",
            display_name: "Rhyhorn",
            national_number: 111,
            types: [PokemonType::Ground, PokemonType::Rock],
            base_stats: [80, 85, 95, 30, 30, 25],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::Slow,
            base_exp_yield: 69,
            ev_yield: [0, 0, 1, 0, 0, 0],
            capture_rate: 120,
            abilities: ["LightningRod", "RockHead"],
            hidden_abilities: ["Reckless"],
            move_table: [
                1: "Tackle",
                1: "TailWhip",
                // 5: "SmackDown",
                // 10: "Bulldoze",
                15: "HornAttack",
                // 20: "ScaryFace",
                // 25: "Stomp",
                // 30: "RockBlast",
                // 35: "Drillrun",
                // 40: "TakeDown",
                // 45: "Earthquake",
                // 50: "StoneEdge",
                // 55: "Megahorn",
                // 60: "HornDrill",
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
