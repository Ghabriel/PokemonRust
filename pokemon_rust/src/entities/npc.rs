use amethyst::{
    assets::ProgressCounter,
    ecs::{Component, DenseVecStorage, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::{
        CommonResources,
        Direction,
        get_character_sprite_index_from_direction,
        load_sprite_sheet_with_texture,
    },
    entities::{
        AnimationData,
        AnimationTable,
        CharacterAnimation,
        character::{Character, StepKind},
    },
    map::{MapCoordinates, MapHandler, PlayerCoordinates, TileData},
};

use serde::{Deserialize, Serialize};

pub struct NpcBuilder {
    pub map_id: String,
    pub position: MapCoordinates,
    pub kind: String,
    pub facing_direction: Direction,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Npc {
    pub action: NpcAction,
    pub kind: String,
}

impl Component for Npc {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NpcAction {
    Moving,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NpcAnimation {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
}

impl From<NpcAnimation> for CharacterAnimation {
    fn from(animation: NpcAnimation) -> CharacterAnimation {
        match animation {
            NpcAnimation::IdleUp => CharacterAnimation::IdleUp,
            NpcAnimation::IdleDown => CharacterAnimation::IdleDown,
            NpcAnimation::IdleLeft => CharacterAnimation::IdleLeft,
            NpcAnimation::IdleRight => CharacterAnimation::IdleRight,
            NpcAnimation::WalkUp => CharacterAnimation::NpcMoveUp,
            NpcAnimation::WalkDown => CharacterAnimation::NpcMoveDown,
            NpcAnimation::WalkLeft => CharacterAnimation::NpcMoveLeft,
            NpcAnimation::WalkRight => CharacterAnimation::NpcMoveRight,
        }
    }
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
        facing_direction: npc_builder.facing_direction,
        next_step: StepKind::Left,
    };

    let npc = Npc {
        action: NpcAction::Moving,
        kind: npc_builder.kind,
    };

    let sprite_render = {
        let resources = world.read_resource::<CommonResources>();

        SpriteRender {
            sprite_sheet: load_sprite_sheet_with_texture(
                world,
                resources.npc_texture.clone(),
                &format!("sprites/characters/{}/spritesheet.ron", npc.kind),
                progress_counter,
            ),
            sprite_number: get_character_sprite_index_from_direction(&character.facing_direction),
        }
    };

    let animation_set = get_npc_animation_set();

    world.register::<AnimationTable<CharacterAnimation>>();
    world.register::<Npc>();

    let entity = world
        .create_entity()
        .with(character)
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

    let idle_animation_timing = vec![1.0];
    let walk_animation_timing = vec![0.1, 0.2, 0.3, 0.4];

    animation_table.insert(NpcAnimation::IdleDown.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![3],
    });
    animation_table.insert(NpcAnimation::IdleLeft.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![6],
    });
    animation_table.insert(NpcAnimation::IdleRight.into(), AnimationData {
        timings: idle_animation_timing.clone(),
        frames: vec![9],
    });
    animation_table.insert(NpcAnimation::IdleUp.into(), AnimationData {
        timings: idle_animation_timing,
        frames: vec![0],
    });

    animation_table.insert(NpcAnimation::WalkDown.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![3, 4, 3, 5],
    });
    animation_table.insert(NpcAnimation::WalkLeft.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![6, 7, 6, 8],
    });
    animation_table.insert(NpcAnimation::WalkRight.into(), AnimationData {
        timings: walk_animation_timing.clone(),
        frames: vec![9, 10, 9, 11],
    });
    animation_table.insert(NpcAnimation::WalkUp.into(), AnimationData {
        timings: walk_animation_timing,
        frames: vec![0, 1, 0, 2],
    });

    animation_table
}
