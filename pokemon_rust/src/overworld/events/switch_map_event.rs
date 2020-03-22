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
    map::{
        change_player_tile,
        prepare_warp,
        MapCoordinates,
        MapHandler,
        PlayerCoordinates,
        TileDataBuilder,
    },
    overworld::{
        entities::character::PlayerEntity,
        events::EventQueue,
    },
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
        T: Into<String>,
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
            ..ExecutionConditions::default()
        }
    }

    fn start(&mut self, world: &mut World) {
        let entity = world.read_resource::<PlayerEntity>().0;

        let character_id = world
            .read_resource::<MapHandler>()
            .get_character_id_by_entity(entity);

        let player_coordinates = world
            .read_storage::<Transform>()
            .get(entity)
            .map(PlayerCoordinates::from_transform)
            .expect("Failed to retrieve Transform");

        let initial_tile_data = TileDataBuilder::default()
            .with_character_id(character_id)
            .with_player_coordinates(player_coordinates)
            .build(world);

        let target_tile_data =
            prepare_warp(world, &self.map, &self.tile, &mut self.progress_counter);

        let target_transform = target_tile_data.position.to_transform();

        world
            .write_storage::<Transform>()
            .get_mut(entity)
            .unwrap()
            .set_translation(*target_transform.translation());

        change_player_tile(
            &initial_tile_data,
            &target_tile_data,
            &world.read_resource::<PlayerEntity>(),
            &mut world.write_resource::<MapHandler>(),
            &mut world.write_resource::<EventQueue>(),
        );
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, _world: &mut World) -> bool {
        self.progress_counter.is_complete()
    }
}
