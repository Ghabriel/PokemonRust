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
    utils::application_root_dir,
};

use crate::{
    common::{
        Direction,
        get_character_sprite_index_from_direction,
        load_sprite_sheet,
    },
    entities::{
        AnimationData,
        AnimationTable,
    },
    map::{
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
        TileData,
    },
};

use ron::de::from_reader;

use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::File,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializableCharacter {
    texture_file_name: String,
    allowed_movements: HashMap<MovementType, SerializableMovementData>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SerializableMovementData {
    sprite_sheet: String,
    velocity: f32,
}

/// Represents the ID of a character.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CharacterId(pub usize);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Character {
    pub action: MovementType,
    pub facing_direction: Direction,
    pub next_step: StepKind,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

/// A resource that's present in the world whenever there's a pending
/// interaction with an NPC.
pub struct PendingInteraction {
    pub character_id: CharacterId,
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
    pub initial_action: MovementType,
}

pub fn initialise_npc(
    world: &mut World,
    npc_builder: NpcBuilder,
    progress_counter: &mut ProgressCounter,
) -> CharacterId {
    let character_data = read_character_file(&npc_builder.kind);

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
        action: npc_builder.initial_action,
        facing_direction: npc_builder.facing_direction,
        next_step: StepKind::Left,
    };

    let mut default_sprite_sheet = None;

    let (animation_table, allowed_movements) = {
        let mut animation_table = AnimationTable::new();
        let mut allowed_movements = AllowedMovements::default();

        add_idle_animations(&mut animation_table);

        for (movement_type, movement_data) in character_data.allowed_movements {
            match movement_type {
                MovementType::Walk => add_walk_animations(&mut animation_table),
                MovementType::Run => add_run_animations(&mut animation_table),
            }

            let texture_file_name = format!(
                "characters/{}/{}",
                npc_builder.kind,
                character_data.texture_file_name,
            );

            let sprite_sheet_file_name = format!(
                "characters/{}/{}",
                npc_builder.kind,
                movement_data.sprite_sheet,
            );

            let sprite_sheet = load_sprite_sheet(
                world,
                &texture_file_name,
                &sprite_sheet_file_name,
                progress_counter,
            );

            if movement_type == character.action {
                default_sprite_sheet = Some(sprite_sheet.clone());
            }

            allowed_movements.add_movement_type(movement_type, MovementData {
                sprite_sheet,
                velocity: movement_data.velocity,
            });
        }

        (animation_table, allowed_movements)
    };

    if default_sprite_sheet.is_none() {
        panic!("Invalid initial action for NPC of kind {}", npc_builder.kind);
    }

    let sprite_render = SpriteRender {
        sprite_sheet: default_sprite_sheet.unwrap(),
        sprite_number: get_character_sprite_index_from_direction(&character.facing_direction),
    };


    world.register::<AnimationTable<CharacterAnimation>>();
    world.register::<AllowedMovements>();
    world.register::<Character>();

    let entity = world
        .create_entity()
        .with(character)
        .with(allowed_movements)
        .with(transform)
        .with(sprite_render)
        .with(animation_table)
        .build();

    world.write_resource::<MapHandler>()
        .register_npc(&map_id, npc_builder.position, entity)
}

fn read_character_file(character_kind: &str) -> SerializableCharacter {
    let character_file = application_root_dir()
        .unwrap()
        .join("assets")
        .join("characters")
        .join(character_kind)
        .join("character.ron");

    let file = File::open(character_file).expect("Failed opening character file");

    from_reader(file).expect("Failed deserializing character")
}

pub fn initialise_player(
    world: &mut World,
    starting_map: &str,
    starting_position: MapCoordinates,
    progress_counter: &mut ProgressCounter,
) -> Entity {
    let player_id = initialise_npc(
        world,
        NpcBuilder {
            map_id: starting_map.to_string(),
            position: starting_position,
            kind: "lucas".to_string(),
            facing_direction: Direction::Down,
            initial_action: MovementType::Walk,
        },
        progress_counter
    );

    world.read_resource::<MapHandler>().get_character_by_id(player_id)
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
