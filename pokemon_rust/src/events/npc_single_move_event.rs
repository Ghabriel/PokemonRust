use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    constants::TILE_SIZE,
    entities::npc::{Npc, NpcMovement},
    map::{MapHandler, PlayerCoordinates, TileData},
};

use super::{GameEvent, ShouldDisableInput};

#[derive(Clone)]
pub struct NpcSingleMoveEvent {
    npc_id: usize,
}

impl NpcSingleMoveEvent {
    pub fn new(npc_id: usize) -> NpcSingleMoveEvent {
        NpcSingleMoveEvent {
            npc_id,
        }
    }
}

impl GameEvent for NpcSingleMoveEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        let map_handler = world.read_resource::<MapHandler>();
        let npc_entity = map_handler.get_npc_by_id(self.npc_id);

        let npcs = world.read_storage::<Npc>();
        let npc = npcs.get(*npc_entity).unwrap();

        let npc_position = world.read_storage::<Transform>()
            .get(*npc_entity)
            .map(PlayerCoordinates::from_transform)
            .unwrap();

        let movement = NpcMovement {
            // TODO: extract velocity to constant or use GameConfig::player_walking_speed
            estimated_time: f32::from(TILE_SIZE) / 160.,
            step_kind: npc.next_step.clone(),
            started: false,
            from: TileData {
                position: npc_position.clone(),
                // TODO: use the NPC's natural map
                map_id: map_handler.get_current_map_id(),
            },
            // TODO: use the NPC's natural map
            to: map_handler.get_forward_tile(&npc.facing_direction, &npc_position),
        };

        world.write_storage::<NpcMovement>()
            .insert(*npc_entity, movement)
            .expect("Failed to attach NpcMovement");

        ShouldDisableInput(false)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }

    fn is_complete(&self, world: &mut World) -> bool {
        let map_handler = world.read_resource::<MapHandler>();
        let npc_entity = map_handler.get_npc_by_id(self.npc_id);

        !world.read_storage::<NpcMovement>()
            .contains(*npc_entity)
    }
}
