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
    MovingUp,
    MovingDown,
    MovingLeft,
    MovingRight,
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

    world.register::<Npc>();

    let entity = world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .build();

    world.write_resource::<MapHandler>()
        .register_npc(&map_id, &npc_builder.position, entity)
}
