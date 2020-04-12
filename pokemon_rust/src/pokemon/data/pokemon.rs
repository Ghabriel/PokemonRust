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
                12: "RazorLeaf",
                15: "PoisonPowder",
                15: "SleepPowder",
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
                4: "Ember",
                // 8: "Smokescreen",
                // 12: "DragonBreath",
                // 17: "FireFang",
                20: "Slash",
                24: "Flamethrower",
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
                13: "Bubble",
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
                // 9: "BugBite",
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
            id: "Butterfree",
            display_name: "Butterfree",
            national_number: 12,
            types: [PokemonType::Bug, PokemonType::Flying],
            base_stats: [60, 45, 50, 90, 80, 70],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 178,
            ev_yield: [0, 0, 0, 2, 1, 0],
            capture_rate: 45,
            abilities: ["CompoundEyes"],
            hidden_abilities: ["TintedLens"],
            move_table: [
                1: "Gust",
                1: "Harden",
                1: "Tackle",
                1: "StringShot",
                // 1: "BugBite",
                // // TODO: learns when evolving
                // 1: "Gust",
                4: "Supersonic",
                8: "Confusion",
                12: "PoisonPowder",
                12: "StunSpore",
                12: "SleepPowder",
                16: "Psybeam",
                // 20: "Whirlwind",
                // 24: "AirSlash",
                // 28: "Safeguard",
                // 32: "BugBuzz",
                // 36: "TailWind",
                // 40: "RagePowder",
                // 44: "QuiverDance",
            ],
        });

        result.push(species! {
            id: "Weedle",
            display_name: "Weedle",
            national_number: 13,
            types: [PokemonType::Bug, PokemonType::Poison],
            base_stats: [40, 35, 30, 20, 20, 50],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 39,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 255,
            abilities: ["ShieldDust"],
            hidden_abilities: ["RunAway"],
            move_table: [
                1: "PoisonSting",
                1: "StringShot",
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
                5: "SandAttack",
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
                21: "HyperFang",
                // 24: "SuckerPunch",
                // 27: "Double-Edge",
            ],
        });

        result.push(species! {
            id: "Spearow",
            display_name: "Spearow",
            national_number: 21,
            types: [PokemonType::Normal, PokemonType::Flying],
            base_stats: [40, 60, 30, 31, 31, 70],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 52,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 255,
            abilities: ["KeenEye"],
            hidden_abilities: ["Sniper"],
            move_table: [
                1: "Peck",
                3: "Growl",
                8: "Leer",
                // 11: "FocusEnergy",
                // 16: "FuryAttack",
                // 19: "MirrorMove",
                // 24: "Roost",
                // 27: "Agility",
                32: "DrillPeck",
            ],
        });

        result.push(species! {
            id: "Pikachu",
            display_name: "Pikachu",
            national_number: 25,
            types: [PokemonType::Electric],
            base_stats: [35, 55, 40, 50, 50, 90],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 112,
            ev_yield: [0, 0, 0, 0, 0, 2],
            capture_rate: 190,
            abilities: ["Static"],
            hidden_abilities: ["LightningRod"],
            move_table: [
                // 1: "PlayNice",
                // 1: "SweetKiss",
                // 1: "Nuzzle",
                // 1: "NastyPlot",
                // 1: "Charm",
                1: "ThunderShock",
                1: "TailWhip",
                1: "Growl",
                1: "QuickAttack",
                4: "ThunderWave",
                // 8: "DoubleTeam",
                // 12: "ElectroBall",
                // 16: "Feint",
                // 20: "Spark",
                // 24: "Agility",
                // 28: "Slam",
                // 32: "Discharge",
                36: "Thunderbolt",
                // 40: "LightScreen",
                // 44: "Thunder",
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
                1: "Sing",
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
            id: "Vulpix",
            display_name: "Vulpix",
            national_number: 37,
            types: [PokemonType::Fire],
            base_stats: [38, 41, 40, 50, 65, 65],
            male_ratio: Some(25.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 113,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 190,
            abilities: ["FlashFire"],
            hidden_abilities: ["Drought"],
            move_table: [
                1: "Ember",
                1: "TailWhip",
                // 4: "Disable",
                8: "QuickAttack",
                // 12: "Spite",
                // 16: "Incinerate",
                // 20: "ConfuseRay",
                // 24: "WillOWisp",
                // 28: "ExtraSensory",
                // 32: "Flamethrower",
                // 36: "Imprison",
                // 40: "FireSpin",
                // 44: "Safeguard",
                // 48: "Inferno",
                // 52: "Grudge",
                56: "FireBlast",
            ],
        });

        result.push(species! {
            id: "Oddish",
            display_name: "Oddish",
            national_number: 43,
            types: [PokemonType::Grass, PokemonType::Poison],
            base_stats: [45, 50, 55, 75, 65, 30],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumSlow,
            base_exp_yield: 64,
            ev_yield: [0, 0, 0, 1, 0, 0],
            capture_rate: 255,
            abilities: ["Chlorophyll"],
            hidden_abilities: ["RunAway"],
            move_table: [
                // 1: "Absorb",
                // 1: "Growth",
                4: "Acid",
                // 8: "SweetScent",
                // 12: "MegaDrain",
                14: "PoisonPowder",
                16: "StunSpore",
                18: "SleepPowder",
                // 20: "GigaDrain",
                // 24: "Toxic",
                // 28: "MoonBlast",
                // 32: "GrassyTerrain",
                // 36: "Moonlight",
                // 40: "PetalDance",
            ],
        });

        result.push(species! {
            id: "Diglett",
            display_name: "Diglett",
            national_number: 50,
            types: [PokemonType::Ground],
            base_stats: [10, 55, 25, 35, 45, 95],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 53,
            ev_yield: [0, 0, 0, 0, 0, 1],
            capture_rate: 255,
            abilities: ["SandVeil", "ArenaTrap"],
            hidden_abilities: ["SandForce"],
            move_table: [
                1: "SandAttack",
                1: "Scratch",
                4: "Growl",
                // 8: "Astonish",
                // 12: "MudSlap",
                // 16: "Bulldoze",
                // 20: "SuckerPunch",
                // 24: "Slash",
                // 28: "Sandstorm",
                // 32: "Dig",
                // 36: "EarthPower",
                // 40: "Earthquake",
                44: "Fissure",
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
                1: "Leer",
                // 4: "FocusEnergy",
                7: "KarateChop",
                // 8: "Revenge",
                // 12: "LowSweep",
                // 16: "KnockOff",
                // 20: "ScaryFace",
                // 24: "VitalThrow",
                29: "Strength",
                // 32: "DualChop",
                // 36: "BulkUp",
                // 40: "SeismicToss",
                // 44: "DynamicPunch",
                // 48: "CrossChop",
                // 52: "DoubleEdge",
            ],
        });

        result.push(species! {
            id: "Tentacool",
            display_name: "Tentacool",
            national_number: 72,
            types: [PokemonType::Water, PokemonType::Poison],
            base_stats: [40, 40, 35, 50, 100, 70],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::Slow,
            base_exp_yield: 67,
            ev_yield: [0, 0, 0, 0, 1, 0],
            capture_rate: 190,
            abilities: ["ClearBody", "LiquidOoze"],
            hidden_abilities: ["RainDish"],
            move_table: [
                1: "PoisonSting",
                4: "Constrict",
                9: "Supersonic",
                13: "Acid",
                18: "BubbleBeam",
                // 22: "Wrap",
                27: "Surf",
                // 31: "Barrier",
                // 36: "PoisonJab",
                // 40: "Screech",
                45: "HydroPump",
            ],
        });

        result.push(species! {
            id: "Slowpoke",
            display_name: "Slowpoke",
            national_number: 79,
            types: [PokemonType::Water, PokemonType::Psychic],
            base_stats: [90, 65, 65, 40, 40, 15],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 63,
            ev_yield: [1, 0, 0, 0, 0, 0],
            capture_rate: 190,
            abilities: ["Oblivious", "OwnTempo"],
            hidden_abilities: ["Regenerator"],
            move_table: [
                1: "Tackle",
                // 1: "Curse",
                3: "Growl",
                6: "WaterGun",
                // 9: "Yawn",
                // 12: "Confusion",
                // 15: "Disable",
                // 18: "WaterPulse",
                21: "Headbutt",
                // 24: "ZenHeadbutt",
                // 27: "Amnesia",
                // 30: "Surf",
                // 33: "SlackOff",
                36: "Psychic",
                // 39: "PsychUp",
                // 42: "RainDance",
                // 45: "HealPulse",
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
                5: "Leer",
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
            id: "Onix",
            display_name: "Onix",
            national_number: 95,
            types: [PokemonType::Rock, PokemonType::Ground],
            base_stats: [35, 45, 160, 30, 45, 70],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 77,
            ev_yield: [0, 0, 1, 0, 0, 0],
            capture_rate: 45,
            abilities: ["RockHead", "Sturdy"],
            hidden_abilities: ["WeakArmor"],
            move_table: [
                1: "Tackle",
                1: "Harden",
                // 1: "Bind",
                1: "RockThrow",
                // 4: "SmackDown",
                // 8: "RockPolish",
                // 12: "DragonBreath",
                // 16: "Curse",
                20: "RockSlide",
                // 24: "Screech",
                // 28: "SandTomb",
                // 32: "StealthRock",
                // 36: "Slam",
                // 40: "Sandstorm",
                // 44: "Dig",
                // 48: "IronTail",
                // 52: "StoneEdge",
                // 56: "DoubleEdge",
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
                1: "Leer",
                4: "Harden",
                // 8: "MetalClaw",
                // 12: "MudShot",
                // 16: "Protect",
                20: "BubbleBeam",
                // 24: "Stomp",
                // 29: "Flail",
                // 32: "RazorShell",
                36: "Slam",
                40: "SwordsDance",
                44: "CrabHammer",
                48: "Guillotine",
            ],
        });

        result.push(species! {
            id: "Exeggcutor",
            display_name: "Exeggcutor",
            national_number: 103,
            types: [PokemonType::Grass, PokemonType::Psychic],
            base_stats: [95, 95, 85, 125, 75, 55],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::Slow,
            base_exp_yield: 186,
            ev_yield: [0, 0, 0, 2, 0, 0],
            capture_rate: 45,
            abilities: ["Chlorophyll"],
            hidden_abilities: ["Harvest"],
            move_table: [
                // 1: "Stomp",
                // 1: "PowerWhip",
                1: "EggBomb",
                // 1: "Barrage",
                1: "Hypnosis",
                // 1: "Confusion",
                1: "StunSpore",
                // // TODO: learns when evolving
                // 1: "Stomp",
            ],
        });

        result.push(species! {
            id: "Cubone",
            display_name: "Cubone",
            national_number: 104,
            types: [PokemonType::Ground],
            base_stats: [50, 50, 95, 40, 50, 35],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 64,
            ev_yield: [0, 0, 1, 0, 0, 0],
            capture_rate: 190,
            abilities: ["RockHead", "LightningRod"],
            hidden_abilities: ["BattleArmor"],
            move_table: [
                1: "Growl",
                2: "TailWhip",
                6: "Leer",
                12: "BoneClub",
                14: "Headbutt",
                // 18: "Rage",
                // 24: "FocusEnergy",
                // 26: "Bonemerang",
                // 30: "Thrash",
                // 36: "DoubleEdge",
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
                4: "DoubleKick",
                5: "Meditate",
                // 8: "LowKick",
                9: "RollingKick",
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
                1: "CometPunch",
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
                24: "ThunderPunch",
                24: "IcePunch",
                24: "FirePunch",
                25: "DizzyPunch",
                // 28: "Agility",
                32: "MegaPunch",
                // 36: "CloseCombat",
                // 40: "Counter",
                // 44: "FocusPunch",
            ],
        });

        result.push(species! {
            id: "Lickitung",
            display_name: "Lickitung",
            national_number: 108,
            types: [PokemonType::Normal],
            base_stats: [90, 55, 75, 60, 75, 30],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 77,
            ev_yield: [2, 0, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["OwnTempo", "Oblivious"],
            hidden_abilities: ["CloudNine"],
            move_table: [
                1: "Lick",
                // 5: "Wrap",
                10: "Acid",
                // 15: "Stomp",
                // 20: "Disable",
                // 25: "Bind",
                // 30: "Slam",
                // 35: "Screech",
                // 40: "Thrash",
                // 45: "PowerWhip",
            ],
        });

        result.push(species! {
            id: "Koffing",
            display_name: "Koffing",
            national_number: 109,
            types: [PokemonType::Poison],
            base_stats: [40, 65, 95, 60, 45, 35],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 68,
            ev_yield: [0, 0, 1, 0, 0, 0],
            capture_rate: 190,
            abilities: ["Levitate", "NeutralizingGas"],
            hidden_abilities: ["Stench"],
            move_table: [
                // 1: "PoisonGas",
                1: "Tackle",
                4: "Smog",
                // 8: "Smokescreen",
                // 12: "ClearSmog",
                // 16: "Assurance",
                20: "Sludge",
                // 24: "Haze",
                // 28: "SelfDestruct",,
                // 32: "SludgeBomb",
                // 36: "Toxic",
                // 40: "Belch",
                // 44: "Explosion",
                // 48: "Memento",
                // 52: "DestinyBond",
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
                60: "HornDrill",
            ],
        });

        result.push(species! {
            id: "Gyarados",
            display_name: "Gyarados",
            national_number: 130,
            types: [PokemonType::Water, PokemonType::Flying],
            base_stats: [95, 125, 79, 60, 100, 81],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::Slow,
            base_exp_yield: 189,
            ev_yield: [0, 2, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["Intimidate"],
            hidden_abilities: ["Moxie"],
            move_table: [
                1: "Bite",
                // 1: "Flail",
                // 1: "Splash",
                1: "Tackle",
                1: "Leer",
                // 1: "Twister",
                // TODO: Learns when evolving
                // 1: "Bite",
                // 4: "Whirlpool",
                // 8: "IceFang",
                // 12: "Brine",
                // 16: "ScaryFace",
                21: "Waterfall",
                // 24: "Crunch",
                // 28: "RainDance",
                // 32: "AquaTail",
                // 36: "DragonDance",
                40: "HydroPump",
                // 44: "Hurricane",
                // 48: "Thrash",
                // 52: "HyperBeam",
            ],
        });

        result.push(species! {
            id: "Lapras",
            display_name: "Lapras",
            national_number: 131,
            types: [PokemonType::Water, PokemonType::Ice],
            base_stats: [130, 85, 80, 85, 95, 60],
            male_ratio: Some(50.),
            growth_rate: GrowthRate::Slow,
            base_exp_yield: 187,
            ev_yield: [2, 0, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["WaterAbsorb", "ShellArmor"],
            hidden_abilities: ["Hydration"],
            move_table: [
                1: "Growl",
                1: "WaterGun",
                5: "Sing",
                // 10: "Mist",
                // 15: "LifeDew",
                // 20: "IceShard",
                // 25: "ConfuseRay",
                // 30: "WaterPulse",
                // 35: "Brine",
                // 40: "BodySlam",
                45: "IceBeam",
                // 50: "RainDance",
                55: "HydroPump",
                // 60: "PerishSong",
                // 65: "SheerCold",
            ],
        });

        result.push(species! {
            id: "Eevee",
            display_name: "Eevee",
            national_number: 133,
            types: [PokemonType::Normal],
            base_stats: [55, 55, 50, 45, 65, 55],
            male_ratio: Some(87.5),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 65,
            ev_yield: [0, 0, 0, 0, 1, 0],
            capture_rate: 45,
            abilities: ["RunAway", "Adaptability"],
            hidden_abilities: ["Anticipation"],
            move_table: [
                // 1: "Covet",
                // 1: "HelpingHand",
                1: "Tackle",
                1: "Growl",
                1: "TailWhip",
                5: "SandAttack",
                10: "QuickAttack",
                // 15: "BabyDollEyes",
                20: "Swift",
                // 25: "Bite",
                // 30: "Copycat",
                // 35: "BatonPass",
                // 40: "TakeDown",
                // 45: "Charm",
                // 50: "DoubleEdge",
                // 55: "LastResort",
            ],
        });

        result.push(species! {
            id: "Vaporeon",
            display_name: "Vaporeon",
            national_number: 134,
            types: [PokemonType::Water],
            base_stats: [130, 65, 60, 110, 95, 65],
            male_ratio: Some(87.5),
            growth_rate: GrowthRate::MediumFast,
            base_exp_yield: 184,
            ev_yield: [2, 0, 0, 0, 0, 0],
            capture_rate: 45,
            abilities: ["WaterAbsorb"],
            hidden_abilities: ["Hydration"],
            move_table: [
                1: "WaterGun",
                // 1: "Covet",
                // 1: "Swift",
                // 1: "Bite",
                // 1: "Copycat",
                // 1: "BatonPass",
                // 1: "TakeDown",
                // 1: "Charm",
                // 1: "DoubleEdge",
                // 1: "HelpingHand",
                1: "Tackle",
                1: "Growl",
                1: "TailWhip",
                // TODO: learns by evolving
                1: "WaterGun",
                5: "SandAttack",
                10: "QuickAttack",
                // 15: "BabyDollEyes",
                // 20: "Haze",
                // 25: "WaterPulse",
                30: "AuroraBeam",
                // 35: "AquaRing",
                // 40: "MuddyWater",
                // 45: "AcidArmor",
                50: "HydroPump",
                // 55: "LastResort",
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
