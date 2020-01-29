use amethyst::{
    assets::ProgressCounter,
    core::Transform,
    ecs::{world::Builder, WorldExt},
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
        npc::{Npc, NpcAction},
        player::PlayerEntity,
    },
    map::{
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
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

    context.world.write_storage::<Npc>()
        .get_mut(*npc_entity)
        .unwrap()
        .facing_direction = direction;
}

pub(super) fn add_npc(context: &mut ExecutionContext, npc_key: usize) -> usize {
    let npc_entity = context.remove::<NpcEntity>(npc_key);

    context.world.register::<Npc>();

    let (map_id, transform) = {
        let map_handler = context.world.read_resource::<MapHandler>();
        let map_id = map_handler.make_map_id(npc_entity.map_id);

        // TODO: use the appropriate coordinate system for NPC positions
        let transform = PlayerCoordinates::from_world_coordinates(
            &map_handler.map_to_world_coordinates(&map_id, &npc_entity.position)
        )
        .to_transform();

        (map_id, transform)
    };

    let npc = Npc {
        action: NpcAction::Idle,
        facing_direction: npc_entity.facing_direction,
        moving: false,
        kind: npc_entity.kind,
    };

    let sprite_render = {
        let resources = context.world.read_resource::<CommonResources>();

        SpriteRender {
            sprite_sheet: load_sprite_sheet_with_texture(
                context.world,
                resources.npc_texture.clone(),
                &format!("sprites/characters/{}/spritesheet.ron", npc.kind),
                &mut ProgressCounter::new(),
            ),
            sprite_number: get_character_sprite_index_from_direction(&npc.facing_direction),
        }
    };

    let entity = context.world
        .create_entity()
        .with(npc)
        .with(transform)
        .with(sprite_render)
        .build();

    context.world.write_resource::<MapHandler>()
        .register_npc(&map_id, &npc_entity.position, entity)
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
