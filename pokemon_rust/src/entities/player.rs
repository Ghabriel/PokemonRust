use amethyst::{
    animation::AnimationSet,
    assets::{Handle, ProgressCounter},
    ecs::{
        Component,
        DenseVecStorage,
        Entity,
        FlaggedStorage,
        NullStorage,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{SpriteRender, SpriteSheet},
};

use crate::{
    common::{Direction, get_character_sprite_index_from_direction, load_sprite_sheet},
    config::GameConfig,
    entities::{CharacterAnimation, make_sprite_animation},
    map::{map_to_world_coordinates, MapCoordinates, PlayerCoordinates, WorldCoordinates},
};

use serde::{Deserialize, Serialize};

pub struct SimulatedPlayer(pub Player);

impl Component for SimulatedPlayer {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct StaticPlayer;

impl Component for StaticPlayer {
    type Storage = NullStorage<Self>;
}

/// Resource that stores the entity corresponding to the human player.
pub struct PlayerEntity(pub Entity);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Player {
    pub action: PlayerAction,
    pub facing_direction: Direction,
    pub moving: bool,
}

impl Component for Player {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlayerAction {
    Idle,
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

    let player = Player {
        action: PlayerAction::Walk,
        facing_direction: Direction::Down,
        moving: false,
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
        sprite_number: get_character_sprite_index_from_direction(&player.facing_direction),
    };

    let animation_set = get_player_animation_set(world, progress_counter);

    world.insert(sprite_sheets);

    world.register::<AnimationSet<CharacterAnimation, SpriteRender>>();
    world.register::<Player>();
    world.register::<SimulatedPlayer>();

    world
        .create_entity()
        .with(SimulatedPlayer(player.clone()))
        .with(player)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build()
}

pub fn get_player_animation_set(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> AnimationSet<CharacterAnimation, SpriteRender> {
    let mut animation_set = AnimationSet::new();

    let idle_animation_timing = vec![0.0, 1.0];
    let walk_animation_timing = vec![0.0, 0.1, 0.2, 0.3, 0.4];
    let run_animation_timing = vec![0.0, 0.0625, 0.125, 0.1875, 0.25];

    animation_set.insert(PlayerAnimation::IdleDown.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![3],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::IdleLeft.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![6],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::IdleRight.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![9],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::IdleUp.into(), make_sprite_animation(
        world,
        idle_animation_timing,
        vec![0],
        progress_counter,
    ));

    animation_set.insert(PlayerAnimation::WalkDown.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![3, 4, 3, 5],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkLeft.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![6, 7, 6, 8],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkRight.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![9, 10, 9, 11],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkUp.into(), make_sprite_animation(
        world,
        walk_animation_timing,
        vec![0, 1, 0, 2],
        progress_counter,
    ));

    animation_set.insert(PlayerAnimation::RunDown.into(), make_sprite_animation(
        world,
        run_animation_timing.clone(),
        vec![3, 4, 3, 5],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunLeft.into(), make_sprite_animation(
        world,
        run_animation_timing.clone(),
        vec![6, 7, 6, 8],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunRight.into(), make_sprite_animation(
        world,
        run_animation_timing.clone(),
        vec![9, 10, 9, 11],
        progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunUp.into(), make_sprite_animation(
        world,
        run_animation_timing,
        vec![0, 1, 0, 2],
        progress_counter,
    ));

    animation_set
}
