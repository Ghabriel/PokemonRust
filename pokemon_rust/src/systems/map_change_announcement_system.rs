use amethyst::{
    core::Time,
    ecs::{Entities, Read, System, Write, WriteStorage},
    ui::UiTransform,
};

use crate::entities::map_change_announcement::{
    MapChangeAnnouncementQueue,
    MapChangeAnnouncementState,
};

pub struct MapChangeAnnouncementSystem;

impl<'a> System<'a> for MapChangeAnnouncementSystem {
    type SystemData = (
        Write<'a, MapChangeAnnouncementQueue>,
        WriteStorage<'a, UiTransform>,
        Entities<'a>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut announcements,
        mut ui_transforms,
        entities,
        time,
    ): Self::SystemData) {
        if announcements.is_empty() {
            return;
        }

        let opening_time = 0.4;
        let waiting_time = 3.;
        let closing_time = 0.4;

        let waiting_box_y = -40.;
        let waiting_text_y = -40.;

        if announcements.len() > 1 {
            let announcement = announcements.front_mut().unwrap();

            match announcement.state {
                MapChangeAnnouncementState::Opening => {
                    /*
                     * The way to find the formula below is described as follows.
                     * To make an announcement in the Opening state transition to the
                     * Closing state in a smooth way (that is, without making any "jumps"
                     * in the animation), the progress formula for both states needs to
                     * yield the same value, i.e:
                     *   T / opening_time = 1 - T' / closing_time,
                     *   where
                     *     T is the current announcement.elapsed_time;
                     *     T' is the new announcement.elapsed_time.
                     * We can then isolate T' in the above equation:
                     *   T' / closing_time = 1 - T / opening_time
                     *   T' = closing_time * (1 - T / opening_time)
                     */
                    announcement.elapsed_time =
                        closing_time * (1. - announcement.elapsed_time / opening_time);
                    announcement.state = MapChangeAnnouncementState::Closing;
                },
                MapChangeAnnouncementState::Waiting => {
                    announcement.elapsed_time = 0.;
                    announcement.state = MapChangeAnnouncementState::Closing;
                },
                MapChangeAnnouncementState::Closing => { },
            }
        }

        let announcement = announcements.front_mut().unwrap();

        announcement.elapsed_time += time.delta_seconds();

        let set_box_transform_y = |value: f32, ui_transforms: &mut WriteStorage<UiTransform>| {
            ui_transforms
                .get_mut(announcement.box_entity)
                .expect("Failed to retrieve UiTransform of announcement box")
                .local_y = value;
        };

        let set_text_transform_y = |value: f32, ui_transforms: &mut WriteStorage<UiTransform>| {
            ui_transforms
                .get_mut(announcement.text_entity)
                .expect("Failed to retrieve UiTransform of announcement text")
                .local_y = value;
        };

        match announcement.state {
            MapChangeAnnouncementState::Opening => {
                if announcement.elapsed_time >= opening_time {
                    announcement.elapsed_time = 0.;
                    announcement.state = MapChangeAnnouncementState::Waiting;
                    return;
                }

                let progress = announcement.elapsed_time / opening_time;
                set_box_transform_y(waiting_box_y * progress, &mut ui_transforms);
                set_text_transform_y(waiting_text_y * progress, &mut ui_transforms);
            },
            MapChangeAnnouncementState::Waiting => {
                if announcement.elapsed_time >= waiting_time {
                    announcement.elapsed_time = 0.;
                    announcement.state = MapChangeAnnouncementState::Closing;
                    return;
                }

                set_box_transform_y(waiting_box_y, &mut ui_transforms);
                set_text_transform_y(waiting_text_y, &mut ui_transforms);
            },
            MapChangeAnnouncementState::Closing => {
                if announcement.elapsed_time >= closing_time {
                    entities.delete(announcement.box_entity).expect("Failed to delete announcement box");
                    entities.delete(announcement.text_entity).expect("Failed to delete announcement text");
                    announcements.pop_front();
                    return;
                }

                let progress = announcement.elapsed_time / closing_time;
                set_box_transform_y(waiting_box_y * (1. - progress), &mut ui_transforms);
                set_text_transform_y(waiting_text_y * (1. - progress), &mut ui_transforms);
            },
        }
    }
}
