use amethyst::{
    assets::{Handle, ProgressCounter},
    ecs::{
        Component,
        DenseVecStorage,
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
        character::{Character, StepKind},
    },
    map::{map_to_world_coordinates, MapCoordinates, PlayerCoordinates, TileData, WorldCoordinates},
};

use serde::{Deserialize, Serialize};

/// Resource that stores the entity corresponding to the human player.
pub struct PlayerEntity(pub Entity);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    pub action: PlayerAction,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlayerAction {
    Walk,
    Run,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlayerAnimation {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    RunUp,
    RunDown,
    RunLeft,
    RunRight,
}

impl From<PlayerAnimation> for CharacterAnimation {
    fn from(animation: PlayerAnimation) -> CharacterAnimation {
        match animation {
            PlayerAnimation::IdleUp => CharacterAnimation::IdleUp,
            PlayerAnimation::IdleDown => CharacterAnimation::IdleDown,
            PlayerAnimation::IdleLeft => CharacterAnimation::IdleLeft,
            PlayerAnimation::IdleRight => CharacterAnimation::IdleRight,
            PlayerAnimation::WalkUp => CharacterAnimation::WalkUp,
            PlayerAnimation::WalkDown => CharacterAnimation::WalkDown,
            PlayerAnimation::WalkLeft => CharacterAnimation::WalkLeft,
            PlayerAnimation::WalkRight => CharacterAnimation::WalkRight,
            PlayerAnimation::RunUp => CharacterAnimation::RunUp,
            PlayerAnimation::RunDown => CharacterAnimation::RunDown,
            PlayerAnimation::RunLeft => CharacterAnimation::RunLeft,
            PlayerAnimation::RunRight => CharacterAnimation::RunRight,
        }
    }
}

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
        facing_direction: Direction::Down,
        next_step: StepKind::Left,
    };

    let player = Player {
        action: PlayerAction::Walk,
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
    world.register::<Player>();

    world
        .create_entity()
        .with(character)
        .with(player)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build()
}

pub fn get_player_animation_set() -> AnimationTable<CharacterAnimation> {
    let mut animation_table = AnimationTable::new();

    let idle_animation_timing = vec![1.0];
    let walk_animation_timing = vec![0.1, 0.2, 0.3, 0.4];
    let run_animation_timing = vec![0.0625, 0.125, 0.1875, 0.25];

    animation_table.insert(PlayerAnimation::IdleDown.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![3],
    });
    animation_table.insert(PlayerAnimation::IdleLeft.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![6],
    });
    animation_table.insert(PlayerAnimation::IdleRight.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![9],
    });
    animation_table.insert(PlayerAnimation::IdleUp.into(), AnimationData {
        timings: idle_animation_timing,
        frames: vec![0],
    });

    animation_table.insert(PlayerAnimation::WalkDown.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });
    animation_table.insert(PlayerAnimation::WalkLeft.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });
    animation_table.insert(PlayerAnimation::WalkRight.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });
    animation_table.insert(PlayerAnimation::WalkUp.into(), AnimationData {
        timings: walk_animation_timing,
        frames: vec![0, 1, 0, 2],
    });

    animation_table.insert(PlayerAnimation::RunDown.into(), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });
    animation_table.insert(PlayerAnimation::RunLeft.into(), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });
    animation_table.insert(PlayerAnimation::RunRight.into(), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });
    animation_table.insert(PlayerAnimation::RunUp.into(), AnimationData {
        timings: run_animation_timing,
        frames: vec![0, 1, 0, 2],
    });

    animation_table
}
