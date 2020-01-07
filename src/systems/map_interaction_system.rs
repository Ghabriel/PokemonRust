use amethyst::{
    core::{math::Vector3, Transform},
    ecs::{Join, Read, ReaderId, ReadExpect, ReadStorage, System, SystemData, World, WorldExt},
    shrev::EventChannel,
};

use crate::{
    constants::TILE_SIZE,
    entities::{
        map::{Map, MapEvent},
        player::{Direction, Player},
    },
};

pub struct MapInteractionSystem {
    event_reader: ReaderId<MapEvent>,
}

impl MapInteractionSystem {
    pub fn new(world: &mut World) -> MapInteractionSystem {
        <Self as System<'_>>::SystemData::setup(world);

        MapInteractionSystem {
            event_reader: world.write_resource::<EventChannel<MapEvent>>().register_reader(),
        }
    }

    fn interact(&mut self, (players, transforms, map, _): &<Self as System<'_>>::SystemData) {
        for (player, transform) in (players, transforms).join() {
            let (offset_x, offset_y) = match player.facing_direction {
                Direction::Up => (0., 1.),
                Direction::Down => (0., -1.),
                Direction::Left => (-1., 0.),
                Direction::Right => (1., 0.),
            };

            let tile_size = TILE_SIZE as f32;

            let interacted_position = transform.translation() + Vector3::new(
                offset_x * tile_size,
                offset_y * tile_size,
                0.,
            );

            let tile_coordinates = map.world_to_tile_coordinates(&interacted_position);
            println!("Coordinates: {:?}", tile_coordinates);
        }
    }
}

impl<'a> System<'a> for MapInteractionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, Map>,
        Read<'a, EventChannel<MapEvent>>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        let (_, _, _, event_channel) = &system_data;

        for event in event_channel.read(&mut self.event_reader) {
            match event {
                MapEvent::Interaction => self.interact(&system_data),
            }
        }
    }
}
