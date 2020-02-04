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
    common::{
        CommonResources,
        Direction,
        get_character_sprite_index_from_direction,
        load_sprite_sheet,
        load_sprite_sheet_with_texture,
    },
    config::GameConfig,
    entities::{
        AnimationData,
        AnimationTable,
    },
    map::{
        map_to_world_coordinates,
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
        TileData,
        WorldCoordinates,
    },
};

use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
    pub action: MovementType,
    pub facing_direction: Direction,
    pub next_step: StepKind,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

/// Represents a character movement in progress.
pub struct CharacterMovement {
    /// Stores how much time it will take for the character to reach the target tile.
    pub estimated_time: f32,
    /// Stores the velocity of this movement.
    pub velocity: f32,
    /// The type of this movement. This determines which animation to use.
    pub movement_type: MovementType,
    /// The kind of step the character is doing while moving. This determines at
    /// which point the animation starts.
    pub step_kind: StepKind,
    /// Determines whether processing for this movement has already started.
    pub started: bool,
    /// The source tile.
    pub from: TileData,
    /// The target tile. Must be adjacent to the source tile.
    pub to: TileData,
}

impl Component for CharacterMovement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StepKind {
    Left,
    Right,
}

impl StepKind {
    pub fn invert(&mut self) {
        *self = match *self {
            StepKind::Left => StepKind::Right,
            StepKind::Right => StepKind::Left,
        };
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MovementType {
    Walk,
    Run,
}

#[derive(Eq, Hash, PartialEq)]
pub enum CharacterAnimation {
    Idle(Direction),
    Moving(MovementType, Direction),
}

#[derive(Default)]
pub struct AllowedMovements {
    movements: HashMap<MovementType, MovementData>,
}

impl AllowedMovements {
    pub fn add_movement_type(&mut self, movement_type: MovementType, data: MovementData) {
        self.movements.insert(movement_type, data);
    }

    pub fn can_perform(&self, movement_type: &MovementType) -> bool {
        self.movements.contains_key(movement_type)
    }

    pub fn get_movement_data(&self, movement_type: &MovementType) -> Option<&MovementData> {
        self.movements.get(movement_type)
    }
}

impl Component for AllowedMovements {
    type Storage = DenseVecStorage<Self>;
}

pub struct MovementData {
    pub sprite_sheet: Handle<SpriteSheet>,
    pub velocity: f32,
}

/// Resource that stores the entity corresponding to the human player.
pub struct PlayerEntity(pub Entity);

pub struct NpcBuilder {
    pub map_id: String,
    pub position: MapCoordinates,
    pub kind: String,
    pub facing_direction: Direction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Npc {
    pub kind: String,
}

impl Component for Npc {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_npc(
    world: &mut World,
    npc_builder: NpcBuilder,
    progress_counter: &mut ProgressCounter,
) -> usize {
    let (map_id, transform) = {
        let map_handler = world.read_resource::<MapHandler>();
        let map_id = map_handler.make_map_id(npc_builder.map_id);

        // TODO: use the appropriate coordinate system for NPC positions
        let transform = PlayerCoordinates::from_world_coordinates(
            &map_handler.map_to_world_coordinates(&map_id, &npc_builder.position)
        )
        .to_transform();

        (map_id, transform)
    };

    let character = Character {
        // TODO: store some kind of default capability and use it here
        action: MovementType::Walk,
        facing_direction: npc_builder.facing_direction,
        next_step: StepKind::Left,
    };

    let npc = Npc {
        kind: npc_builder.kind,
    };

    let sprite_sheet = {
        let resources = world.read_resource::<CommonResources>();

        load_sprite_sheet_with_texture(
            world,
            resources.npc_texture.clone(),
            &format!("sprites/characters/{}/spritesheet.ron", npc.kind),
            progress_counter,
        )
    };

    let allowed_movements = {
        let mut result = AllowedMovements::default();

        // TODO: decide the allowed movements based on the NPC kind and/or metadata
        result.add_movement_type(MovementType::Walk, MovementData {
            sprite_sheet: sprite_sheet.clone(),
            // TODO: extract velocity to constant or use GameConfig::player_walking_speed
            velocity: 160.,
        });

        result
    };

    let sprite_render = SpriteRender {
        sprite_sheet,
        sprite_number: get_character_sprite_index_from_direction(&character.facing_direction),
    };

    let animation_set = get_npc_animation_set();

    world.register::<AnimationTable<CharacterAnimation>>();
    world.register::<Npc>();

    let entity = world
        .create_entity()
        .with(character)
        .with(allowed_movements)
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build();

    world.write_resource::<MapHandler>()
        .register_npc(&map_id, &npc_builder.position, entity)
}

pub fn get_npc_animation_set() -> AnimationTable<CharacterAnimation> {
    let mut animation_table = AnimationTable::new();

    add_idle_animations(&mut animation_table);
    add_walk_animations(&mut animation_table);

    animation_table
}

pub fn initialise_player(world: &mut World, progress_counter: &mut ProgressCounter) -> Entity {
    let character = Character {
        action: MovementType::Walk,
        facing_direction: Direction::Down,
        next_step: StepKind::Left,
    };

    let walking_sprite_sheet = load_sprite_sheet(
        world,
        "sprites/characters/lucas/lucas.png",
        "sprites/characters/lucas/lucas-walking.ron",
        progress_counter,
    );

    let allowed_movements = {
        let mut result = AllowedMovements::default();
        let game_config = world.read_resource::<GameConfig>();

        result.add_movement_type(MovementType::Walk, MovementData {
            sprite_sheet: walking_sprite_sheet.clone(),
            velocity: game_config.player_walking_speed,
        });

        result.add_movement_type(MovementType::Run, MovementData {
            sprite_sheet: load_sprite_sheet(
                world,
                "sprites/characters/lucas/lucas.png",
                "sprites/characters/lucas/lucas-running.ron",
                progress_counter,
            ),
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
        sprite_sheet: walking_sprite_sheet,
        sprite_number: get_character_sprite_index_from_direction(&character.facing_direction),
    };

    let animation_set = get_player_animation_set();

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

    add_idle_animations(&mut animation_table);
    add_walk_animations(&mut animation_table);
    add_run_animations(&mut animation_table);

    animation_table
}

pub fn add_idle_animations(animation_table: &mut AnimationTable<CharacterAnimation>) {
    let idle_animation_timing = vec![1.0];

    animation_table.insert(CharacterAnimation::Idle(Direction::Down), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![3],
    });

    animation_table.insert(CharacterAnimation::Idle(Direction::Left), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![6],
    });

    animation_table.insert(CharacterAnimation::Idle(Direction::Right), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![9],
    });

    animation_table.insert(CharacterAnimation::Idle(Direction::Up), AnimationData {
        timings: idle_animation_timing,
        frames: vec![0],
    });
}

pub fn add_walk_animations(animation_table: &mut AnimationTable<CharacterAnimation>) {
    let walk_animation_timing = vec![0.1, 0.2, 0.3, 0.4];

    animation_table.insert(CharacterAnimation::Moving(MovementType::Walk, Direction::Down), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Walk, Direction::Left), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Walk, Direction::Right), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Walk, Direction::Up), AnimationData {
        timings: walk_animation_timing,
        frames: vec![0, 1, 0, 2],
    });
}

pub fn add_run_animations(animation_table: &mut AnimationTable<CharacterAnimation>) {
    let run_animation_timing = vec![0.0625, 0.125, 0.1875, 0.25];

    animation_table.insert(CharacterAnimation::Moving(MovementType::Run, Direction::Down), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Run, Direction::Left), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Run, Direction::Right), AnimationData {
        timings: run_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });

    animation_table.insert(CharacterAnimation::Moving(MovementType::Run, Direction::Up), AnimationData {
        timings: run_animation_timing,
        frames: vec![0, 1, 0, 2],
    });
}
