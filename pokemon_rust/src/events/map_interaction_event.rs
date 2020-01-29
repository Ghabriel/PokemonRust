use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    entities::player::{Player, PlayerEntity},
    events::EventQueue,
    map::{GameActionKind, MapHandler, ValidatedGameAction},
};

use super::{GameEvent, ShouldDisableInput};

pub struct MapInteractionEvent;

impl GameEvent for MapInteractionEvent {
    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        ShouldDisableInput(false)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let map = world.read_resource::<MapHandler>();
        let player_entity = world.read_resource::<PlayerEntity>();
        let players = world.read_storage::<Player>();
        let transforms = world.read_storage::<Transform>();

        let player = players
            .get(player_entity.0)
            .expect("Failed to retrieve Player");

        let transform = transforms
            .get(player_entity.0)
            .expect("Failed to retrieve Transform");

        let interacted_position = map.get_forward_tile(&player, &transform);

        match map.get_action_at(&interacted_position) {
            Some(
                ValidatedGameAction { when, script_event }
            ) if when == GameActionKind::OnInteraction => {
                world.write_resource::<EventQueue>()
                    .push(script_event);
            },
            _ => {},
        }
    }

    fn is_complete(&self) -> bool {
        true
    }
}
