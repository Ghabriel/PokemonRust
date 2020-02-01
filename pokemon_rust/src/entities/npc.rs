use amethyst::{
    animation::{AnimationCommand, AnimationControlSet, AnimationSet, EndControl, get_animation_set},
    assets::ProgressCounter,
    ecs::{Component, DenseVecStorage, Entity, world::Builder, World, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::{
        CommonResources,
        Direction,
        get_character_sprite_index_from_direction,
        load_sprite_sheet_with_texture,
    },
    entities::{CharacterAnimation, make_sprite_animation},
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
    pub facing_direction: Direction,
    pub moving: bool,
    pub kind: String,
}

impl Component for Npc {
    type Storage = DenseVecStorage<Self>;
}

/// Represents an NPC movement in progress.
pub struct NpcMovement {
    /// Stores how much time it will take for the NPC to reach the target tile.
    pub estimated_time: f32,
    /// Determines whether processing for this movement has already started.
    pub started: bool,
    /// The source tile.
    pub from: TileData,
    /// The target tile. Must be adjacent to the source tile.
    pub to: TileData,
}

impl Component for NpcMovement {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum NpcAction {
    Idle,
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

    let npc = Npc {
        action: NpcAction::Idle,
        facing_direction: npc_builder.facing_direction,
        moving: false,
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
            sprite_number: get_character_sprite_index_from_direction(&npc.facing_direction),
        }
    };

    let animation_set = get_npc_animation_set(world, progress_counter);

    world.register::<AnimationControlSet<CharacterAnimation, SpriteRender>>();
    world.register::<AnimationSet<CharacterAnimation, SpriteRender>>();
    world.register::<Npc>();

    let entity = world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build();

    attach_animation_control_sets(world, entity);

    world.write_resource::<MapHandler>()
        .register_npc(&map_id, &npc_builder.position, entity)
}

pub fn get_npc_animation_set(
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> AnimationSet<CharacterAnimation, SpriteRender> {
    let mut animation_set = AnimationSet::new();

    let idle_animation_timing = vec![0.0, 1.0];
    let walk_animation_timing = vec![0.0, 0.1, 0.2, 0.3, 0.4];

    animation_set.insert(NpcAnimation::IdleDown.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![3],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::IdleLeft.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![6],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::IdleRight.into(), make_sprite_animation(
        world,
        idle_animation_timing.clone(),
        vec![9],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::IdleUp.into(), make_sprite_animation(
        world,
        idle_animation_timing,
        vec![0],
        progress_counter,
    ));

    animation_set.insert(NpcAnimation::WalkDown.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![3, 4, 3, 5],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::WalkLeft.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![6, 7, 6, 8],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::WalkRight.into(), make_sprite_animation(
        world,
        walk_animation_timing.clone(),
        vec![9, 10, 9, 11],
        progress_counter,
    ));
    animation_set.insert(NpcAnimation::WalkUp.into(), make_sprite_animation(
        world,
        walk_animation_timing,
        vec![0, 1, 0, 2],
        progress_counter,
    ));

    animation_set
}

pub fn attach_animation_control_sets(world: &mut World, entity: Entity) {
    let mut control_sets = world.write_storage::<AnimationControlSet<CharacterAnimation, SpriteRender>>();
    let animation_control_set = get_animation_set(&mut control_sets, entity).unwrap();

    let animation_sets = world.read_storage::<AnimationSet<CharacterAnimation, SpriteRender>>();
    let animation_set = animation_sets.get(entity).unwrap();

    let animations = [
        NpcAnimation::IdleUp,
        NpcAnimation::IdleDown,
        NpcAnimation::IdleLeft,
        NpcAnimation::IdleRight,
        NpcAnimation::WalkUp,
        NpcAnimation::WalkDown,
        NpcAnimation::WalkLeft,
        NpcAnimation::WalkRight,
    ];

    for &animation in &animations {
        animation_control_set.add_animation(
            animation.into(),
            &animation_set.get(&animation.into()).unwrap(),
            EndControl::Loop(None),
            1.0,
            AnimationCommand::Init,
        );
    }
}
