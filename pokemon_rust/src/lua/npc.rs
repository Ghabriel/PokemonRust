use amethyst::{
    assets::ProgressCounter,
    core::Transform,
    ecs::WorldExt,
    renderer::SpriteRender,
};

use crate::{
    common::{
        Direction,
        get_character_sprite_index_from_direction,
    },
    entities::character::{Character, initialise_npc, NpcBuilder, PlayerEntity},
    map::{
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
    },
};

use super::ExecutionContext;

pub(super) fn create_npc(
    context: &mut ExecutionContext,
    map_id: String,
    x: u32,
    y: u32,
    kind: String,
    direction: u8,
) -> usize {
    let npc = NpcBuilder {
        map_id,
        position: MapCoordinates::new(x, y),
        kind,
        facing_direction: parse_lua_direction(direction),
    };

    context.store(npc)
}

pub(super) fn change_npc_direction(context: &mut ExecutionContext, npc_key: usize, direction: u8) {
    let mut npc = context.remove::<NpcBuilder>(npc_key);

    npc.facing_direction = parse_lua_direction(direction);

    context.store_at(npc_key, npc);
}

pub(super) fn rotate_npc_towards_player(context: &mut ExecutionContext, npc_id: usize) {
    let map_handler = context.world.write_resource::<MapHandler>();
    let npc_entity = map_handler.get_npc_by_id(npc_id);

    let npc_position = context.world.read_storage::<Transform>()
        .get(*npc_entity)
        .map(PlayerCoordinates::from_transform)
        .unwrap();

    let player_position = {
        let player_entity = context.world.read_resource::<PlayerEntity>();

        context.world.read_storage::<Transform>()
            .get(player_entity.0)
            .map(PlayerCoordinates::from_transform)
            .unwrap()
    };

    let direction = npc_position.get_direction_to(&player_position).unwrap();

    context.world.write_storage::<SpriteRender>()
        .get_mut(*npc_entity)
        .unwrap()
        .sprite_number = get_character_sprite_index_from_direction(&direction);

    context.world.write_storage::<Character>()
        .get_mut(*npc_entity)
        .unwrap()
        .facing_direction = direction;
}

pub(super) fn add_npc(context: &mut ExecutionContext, npc_key: usize) -> usize {
    let npc_builder = context.remove::<NpcBuilder>(npc_key);

    initialise_npc(context.world, *npc_builder, &mut ProgressCounter::new())
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
