//! Signals the game that the interaction button was pressed. If there's
//! an [interaction event](../map/map/enum.GameActionKind.html#variant.OnInteraction)
//! on the tile in front of the player, it is added to the
//! [Event Queue](event_queue/struct.EventQueue.html).

use amethyst::ecs::{SystemData, World, WorldExt};

use crate::{
    audio::{Sound, SoundKit},
    map::{GameActionKind, MapHandler, TileDataBuilder, ValidatedGameAction},
    overworld::{
        entities::character::{Character, PlayerEntity},
        events::EventQueue,
    },
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct MapInteractionEvent;

impl GameEvent for MapInteractionEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
            ..ExecutionConditions::default()
        }
    }

    fn start(&mut self, _world: &mut World) {}

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let player_entity = world.read_resource::<PlayerEntity>().0;

        let tile_data = TileDataBuilder::default()
            .with_entity(player_entity)
            .build(world);

        let map = world.read_resource::<MapHandler>();

        let characters = world.read_storage::<Character>();
        let character = characters
            .get(player_entity)
            .expect("Failed to retrieve Character");

        let interacted_position = map.get_forward_tile(&character.facing_direction, &tile_data);

        match map.get_action_at(&interacted_position) {
            Some(ValidatedGameAction { when, script_event })
                if when == GameActionKind::OnInteraction =>
            {
                SoundKit::fetch(world).play_sound(Sound::SelectOption);

                world.write_resource::<EventQueue>().push(script_event);
            }
            _ => {},
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
