use amethyst::{
    assets::ProgressCounter,
    core::Transform,
    ecs::{World, WorldExt},
    shrev::EventChannel,
};

use crate::{
    entities::{
        map::{change_tile, MapCoordinates, MapHandler, prepare_warp, ScriptEvent},
        player::PlayerEntity,
    },
};

use super::{GameEvent, ShouldDisableInput};

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

impl GameEvent for SwitchMapEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        let starting_map_id = world.read_resource::<MapHandler>().get_current_map_id();
        let target_tile_data = prepare_warp(world, &self.map, &self.tile, &mut self.progress_counter);

        let player_entity = world.read_resource::<PlayerEntity>();

        world.write_storage::<Transform>()
            .get_mut(player_entity.0)
            .expect("Failed to retrieve Transform")
            .set_translation(*target_tile_data.position.to_transform().translation());

        change_tile(
            &starting_map_id,
            &target_tile_data,
            &world.read_resource::<MapHandler>(),
            &mut world.write_resource::<EventChannel<ScriptEvent>>(),
        );

        ShouldDisableInput(true)
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) { }


    fn is_complete(&self) -> bool {
        self.progress_counter.is_complete()
    }
}
