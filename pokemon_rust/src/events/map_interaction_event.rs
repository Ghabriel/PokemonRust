//! Signals the game that the interaction button was pressed. If there's
//! an [interaction event](../map/map/enum.GameActionKind.html#variant.OnInteraction)
//! on the tile in front of the player, it is added to the
//! [Event Queue](event_queue/struct.EventQueue.html).

use amethyst::{
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    entities::{
        character::Character,
        player::PlayerEntity,
    },
    events::EventQueue,
    map::{GameActionKind, MapHandler, PlayerCoordinates, ValidatedGameAction},
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
        let characters = world.read_storage::<Character>();
        let transforms = world.read_storage::<Transform>();

        let character = characters
            .get(player_entity.0)
            .expect("Failed to retrieve Character");

        let transform = transforms
            .get(player_entity.0)
            .expect("Failed to retrieve Transform");

        let interacted_position = map.get_forward_tile(
            &character.facing_direction,
            &PlayerCoordinates::from_transform(&transform),
        );

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

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
