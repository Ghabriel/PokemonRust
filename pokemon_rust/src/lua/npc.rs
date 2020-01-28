use amethyst::{
    ecs::{world::Builder, WorldExt},
    renderer::SpriteRender,
};

use crate::{
    common::Direction,
    entities::{
        map::{map_to_world_coordinates, MapCoordinates, PlayerCoordinates, WorldCoordinates},
        npc::{Npc, NpcAction},
        player::PlayerSpriteSheets,
    },
};

use super::ExecutionContext;

struct NpcEntity {
    position: MapCoordinates,
    kind: String,
    facing_direction: Direction,
}

pub(super) fn create_npc(
    context: &mut ExecutionContext,
    x: u32,
    y: u32,
    kind: String,
    direction: u8,
) -> usize {
    let npc = NpcEntity {
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

    let npc_count = context.world.read_storage::<Npc>().count();

    let npc = Npc {
        // TODO: retrieve the correct map name
        // TODO: find a better ID scheme that guarantees that no conflicts will
        // occur within a map (the current strategy might, if a story event
        // creates or removes NPCs dynamically).
        id: npc_count,
        action: NpcAction::Idle,
        facing_direction: npc_entity.facing_direction,
        moving: false,
        kind: npc_entity.kind,
    };

    let transform = PlayerCoordinates::from_world_coordinates(
        &map_to_world_coordinates(
            &npc_entity.position,
            // TODO: retrieve the correct reference point
            &WorldCoordinates::origin(),
        ),
    )
    .to_transform();

    let sprite_render = {
        let sprite_sheets = context.world.read_resource::<PlayerSpriteSheets>();

        SpriteRender {
            sprite_sheet: sprite_sheets.walking.clone(),
            sprite_number: 0,
        }
    };

    // {
    //     let map_handler: &mut MapHandler = &mut context.world.write_resource::<MapHandler>();
    //     let MapHandler { loaded_maps, current_map } = map_handler;

    //     // TODO: retrieve the correct map
    //     let map = loaded_maps.get_mut(&current_map.0).unwrap();

    //     map.script_repository.push(GameScript::Lua {
    //         file: "assets/maps/test_map/scripts.lua".to_string(),
    //         function: "interact_with_npc".to_string(),
    //         parameters: Some(LuaGameScriptParameters::SourceTile(position.clone())),
    //     });

    //     map.actions.insert(position.clone(), GameAction {
    //         when: GameActionKind::OnInteraction,
    //         script_index: map.script_repository.len() - 1,
    //     });

    //     map.solids.insert(position, Tile);
    // }

    context.world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .build();

    npc_count
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
