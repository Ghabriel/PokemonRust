use amethyst::{
    core::Transform,
    ecs::{
        Join,
        Read,
        ReaderId,
        ReadExpect,
        ReadStorage,
        System,
        SystemData,
        World,
        WorldExt,
        Write,
    },
    shrev::EventChannel,
};

use crate::{
    entities::{
        map::{GameActionKind, MapEvent, MapHandler, ValidatedGameAction},
        player::Player,
    },
    events::EventQueue,
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
}

impl<'a> System<'a> for MapInteractionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, MapHandler>,
        Read<'a, EventChannel<MapEvent>>,
        Write<'a, EventQueue>,
    );

    fn run(&mut self, mut system_data: Self::SystemData) {
        let (_, _, _, map_event_channel, _) = &system_data;

        let events = map_event_channel
            .read(&mut self.event_reader)
            .map(Clone::clone)
            .collect::<Vec<_>>();

        for event in events {
            match event {
                MapEvent::Interaction => interact(&mut system_data),
            }
        }
    }
}

fn interact(system_data: &mut <MapInteractionSystem as System<'_>>::SystemData) {
    let (players, transforms, map, _, event_queue) = system_data;

    for (player, transform) in (&*players, &*transforms).join() {
        let interacted_position = map.get_forward_tile(&player, &transform);

        match map.get_action_at(&interacted_position) {
            Some(
                ValidatedGameAction { when, script_event }
            ) if when == GameActionKind::OnInteraction => {
                event_queue.push(script_event);
            },
            _ => {},
        }
    }
}
