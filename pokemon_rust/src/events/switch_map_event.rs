//! Immediately switches the map that the player is in, without any animations.
//! To fade the screen while the switch is happening, use a
//! [`WarpEvent`](warp_event/struct.WarpEvent.html). This event only finishes
//! when the target map finishes loading.

use amethyst::{
    assets::ProgressCounter,
    core::Transform,
    ecs::{World, WorldExt},
};

use crate::{
    entities::character::PlayerEntity,
    events::EventQueue,
    map::{change_player_tile, MapCoordinates, MapHandler, PlayerCoordinates, prepare_warp, TileData},
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

pub struct SwitchMapEvent {
    map: String,
    tile: MapCoordinates,
    progress_counter: ProgressCounter,
}

impl SwitchMapEvent {
    pub fn new<T>(map: T, tile: MapCoordinates) -> SwitchMapEvent
    where
        T: Into<String>
    {
        SwitchMapEvent {
            map: map.into(),
            tile,
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl Clone for SwitchMapEvent {
    fn clone(&self) -> SwitchMapEvent {
        SwitchMapEvent {
            map: self.map.clone(),
            tile: self.tile.clone(),
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl GameEvent for SwitchMapEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: true,
        }
    }

    fn start(&mut self, world: &mut World) {
        let target_tile_data = prepare_warp(world, &self.map, &self.tile, &mut self.progress_counter);

        let mut transforms = world.write_storage::<Transform>();
        let player_entity = world.read_resource::<PlayerEntity>();

        let transform = transforms
            .get_mut(player_entity.0)
            .expect("Failed to retrieve Transform");

        let initial_tile_data = TileData {
            position: PlayerCoordinates::from_transform(&transform),
            map_id: world.read_resource::<MapHandler>().get_current_map_id(),
        };

        transform.set_translation(*target_tile_data.position.to_transform().translation());

        change_player_tile(
            &initial_tile_data,
            &target_tile_data,
            &world.read_resource::<PlayerEntity>(),
            &mut world.write_resource::<MapHandler>(),
            &mut world.write_resource::<EventQueue>(),
        );
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }


    fn is_complete(&self, _world: &mut World) -> bool {
        self.progress_counter.is_complete()
    }
}
