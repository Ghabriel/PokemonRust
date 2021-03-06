use amethyst::{core::Transform, ecs::WorldExt, renderer::SpriteRender};

use crate::{
    common::{get_character_sprite_index_from_direction, Direction},
    map::{MapCoordinates, MapHandler, PlayerCoordinates},
    overworld::entities::character::{
        initialise_npc,
        Character,
        CharacterId,
        MovementType,
        NpcBuilder,
        PlayerEntity,
    },
};

use super::ExecutionContext;

pub(super) fn create_npc(
    context: &mut ExecutionContext,
    map_id: String,
    x: u32,
    y: u32,
    kind: String,
    direction: Direction,
) -> usize {
    let npc = NpcBuilder {
        map_id,
        position: MapCoordinates::new(x, y),
        kind,
        facing_direction: direction,
        // TODO: this should be a parameter
        initial_action: MovementType::Walk,
    };

    context.store(npc)
}

pub(super) fn change_npc_direction(
    context: &mut ExecutionContext,
    npc_key: usize,
    direction: Direction,
) {
    let mut npc = context.remove::<NpcBuilder>(npc_key);

    npc.facing_direction = direction;

    context.store_at(npc_key, npc);
}

pub(super) fn rotate_npc_towards_player(context: &mut ExecutionContext, character_id: CharacterId) {
    let npc_entity = context
        .world
        .write_resource::<MapHandler>()
        .get_character_by_id(character_id);

    let npc_position = context
        .world
        .read_storage::<Transform>()
        .get(npc_entity)
        .map(PlayerCoordinates::from_transform)
        .unwrap();

    let player_position = {
        let player_entity = context.world.read_resource::<PlayerEntity>();

        context
            .world
            .read_storage::<Transform>()
            .get(player_entity.0)
            .map(PlayerCoordinates::from_transform)
            .unwrap()
    };

    let direction = npc_position.get_direction_to(&player_position).unwrap();

    context
        .world
        .write_storage::<SpriteRender>()
        .get_mut(npc_entity)
        .unwrap()
        .sprite_number = get_character_sprite_index_from_direction(&direction);

    context
        .world
        .write_storage::<Character>()
        .get_mut(npc_entity)
        .unwrap()
        .facing_direction = direction;
}

pub(super) fn add_npc(context: &mut ExecutionContext, npc_key: usize) -> usize {
    let npc_builder = context.remove::<NpcBuilder>(npc_key);

    let character_id = initialise_npc(
        context.world,
        *npc_builder,
        context.asset_tracker.get_progress_counter_mut(),
    );

    character_id.0
}
