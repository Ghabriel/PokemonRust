use amethyst::{
    ecs::{world::Builder, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::Direction,
    entities::{
        map::{
            MapCoordinates,
            MapHandler,
            PlayerCoordinates,
        },
        npc::{Npc, NpcAction},
        player::PlayerSpriteSheets,
    },
};

use super::ExecutionContext;

struct NpcEntity {
    map_id: String,
    position: MapCoordinates,
    kind: String,
    facing_direction: Direction,
}

pub(super) fn create_npc(
    context: &mut ExecutionContext,
    map_id: String,
    x: u32,
    y: u32,
    kind: String,
    direction: u8,
) -> usize {
    let npc = NpcEntity {
        map_id,
        position: MapCoordinates::new(x, y),
        kind,
        facing_direction: parse_lua_direction(direction),
    };

    context.store(npc)
}

pub(super) fn change_npc_direction(context: &mut ExecutionContext, npc_key: usize, direction: u8) {
    let mut npc = context.remove::<NpcEntity>(npc_key);

    npc.facing_direction = parse_lua_direction(direction);

    context.store_at(npc_key, npc);
}

pub(super) fn add_npc(context: &mut ExecutionContext, npc_key: usize) -> usize {
    let npc_entity = context.remove::<NpcEntity>(npc_key);

    context.world.register::<Npc>();

    let (npc_id, transform) = {
        let mut map_handler = context.world.write_resource::<MapHandler>();
        let map_id = map_handler.make_map_id(npc_entity.map_id);

        let npc_id = map_handler.register_npc(&map_id, &npc_entity.position);

        // TODO: use the appropriate coordinate system for NPC positions
        let transform = PlayerCoordinates::from_world_coordinates(
            &map_handler.map_to_world_coordinates(&map_id, &npc_entity.position)
        )
        .to_transform();

        (npc_id, transform)
    };

    let npc = Npc {
        id: npc_id,
        action: NpcAction::Idle,
        facing_direction: npc_entity.facing_direction,
        moving: false,
        kind: npc_entity.kind,
    };

    let sprite_render = {
        let sprite_sheets = context.world.read_resource::<PlayerSpriteSheets>();

        SpriteRender {
            sprite_sheet: sprite_sheets.walking.clone(),
            sprite_number: 0,
        }
    };

    context.world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .build();

    npc_id
}

fn parse_lua_direction(direction: u8) -> Direction {
    match direction {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => panic!("Invalid direction"),
    }
}
