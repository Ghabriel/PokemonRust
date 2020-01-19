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
        map::{GameActionKind, MapEvent, MapHandler, ScriptEvent, ValidatedGameAction},
        player::Player,
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

    fn interact(&mut self, system_data: &mut <Self as System<'_>>::SystemData) {
        let (players, transforms, map, _, script_event_channel) = system_data;

        for (player, transform) in (&*players, &*transforms).join() {
            let interacted_position = map.get_forward_tile(&player, &transform);

            match map.get_action_at(&interacted_position) {
                Some(
                    ValidatedGameAction { when, script_event }
                ) if when == GameActionKind::OnInteraction => {
                    script_event_channel.single_write(script_event);
                },
                _ => {},
            }
        }
    }
}

impl<'a> System<'a> for MapInteractionSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadExpect<'a, MapHandler>,
        Read<'a, EventChannel<MapEvent>>,
        Write<'a, EventChannel<ScriptEvent>>,
    );

    fn run(&mut self, mut system_data: Self::SystemData) {
        let (_, _, _, map_event_channel, _) = &system_data;

        let events = map_event_channel
            .read(&mut self.event_reader)
            .into_iter()
            .map(Clone::clone)
            .collect::<Vec<_>>();

        for event in events {
            match event {
                MapEvent::Interaction => self.interact(&mut system_data),
            }
        }
    }
}
