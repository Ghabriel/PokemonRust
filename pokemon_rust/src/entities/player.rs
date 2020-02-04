use amethyst::{
    assets::{Handle, ProgressCounter},
    ecs::{
        Entity,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    common::{Direction, get_character_sprite_index_from_direction, load_sprite_sheet},
    config::GameConfig,
    entities::{
        AnimationData,
        AnimationTable,
        CharacterAnimation,
        character::{AllowedMovements, Character, MovementData, MovementType, StepKind},
    },
    map::{
        map_to_world_coordinates,
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
        WorldCoordinates,
    },
};

use serde::{Deserialize, Serialize};

/// Resource that stores the entity corresponding to the human player.
pub struct PlayerEntity(pub Entity);

pub struct PlayerSpriteSheets {
    pub walking: Handle<SpriteSheet>,
    pub running: Handle<SpriteSheet>,
}

pub fn initialise_player(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let sprite_sheets = PlayerSpriteSheets {
        walking: load_sprite_sheet(
            world,
            "sprites/characters/lucas/lucas.png",
            "sprites/characters/lucas/lucas-walking.ron",
            progress_counter,
        ),
        running: load_sprite_sheet(
            world,
            "sprites/characters/lucas/lucas.png",
            "sprites/characters/lucas/lucas-running.ron",
            progress_counter,
        ),
    };

    let character = Character {
        action: MovementType::Walk,
        facing_direction: Direction::Down,
        next_step: StepKind::Left,
    };

    let allowed_movements = {
        let mut result = AllowedMovements::default();
        let game_config = world.read_resource::<GameConfig>();

        result.add_movement_type(MovementType::Walk, MovementData {
            sprite_sheet: sprite_sheets.walking.clone(),
            velocity: game_config.player_walking_speed,
        });

        result.add_movement_type(MovementType::Run, MovementData {
            sprite_sheet: sprite_sheets.running.clone(),
            velocity: game_config.player_running_speed,
        });

        result
    };

    let transform = {
        let game_config = world.read_resource::<GameConfig>();
        let position = MapCoordinates::from_tuple(&game_config.player_starting_position);
        let position = map_to_world_coordinates(&position, &WorldCoordinates::origin());

        PlayerCoordinates::from_world_coordinates(&position)
            .to_transform()
    };

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheets.walking.clone(),
        sprite_number: get_character_sprite_index_from_direction(&character.facing_direction),
    };

    let animation_set = get_player_animation_set();

    world.insert(sprite_sheets);

    world.register::<AnimationTable<CharacterAnimation>>();
    world.register::<AllowedMovements>();
    world.register::<Character>();

    let entity = world
        .create_entity()
        .with(character)
        .with(allowed_movements)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build();

    world.write_resource::<MapHandler>()
        .register_player(entity);

    entity
}

pub fn get_player_animation_set() -> AnimationTable<CharacterAnimation> {
    let mut animation_table = AnimationTable::new();

    let idle_animation_timing = vec![1.0];
    let walk_animation_timing = vec![0.1, 0.2, 0.3, 0.4];
    let run_animation_timing = vec![0.0625, 0.125, 0.1875, 0.25];

    animation_table.insert(CharacterAnimation::IdleDown, AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![3],
    });
    animation_table.insert(CharacterAnimation::IdleLeft, AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![6],
    });
    animation_table.insert(CharacterAnimation::IdleRight, AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![9],
    });
    animation_table.insert(CharacterAnimation::IdleUp, AnimationData {
        timings: idle_animation_timing,
        frames: vec![0],
    });

    animation_table.insert(CharacterAnimation::WalkDown, AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });
    animation_table.insert(CharacterAnimation::WalkLeft, AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });
    animation_table.insert(CharacterAnimation::WalkRight, AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });
    animation_table.insert(CharacterAnimation::WalkUp, AnimationData {
        timings: walk_animation_timing,
        frames: vec![0, 1, 0, 2],
    });

    animation_table.insert(CharacterAnimation::RunDown, AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });
    animation_table.insert(CharacterAnimation::RunLeft, AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });
    animation_table.insert(CharacterAnimation::RunRight, AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });
    animation_table.insert(CharacterAnimation::RunUp, AnimationData {
        timings: run_animation_timing,
        frames: vec![0, 1, 0, 2],
    });

    animation_table
}
