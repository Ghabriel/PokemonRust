use amethyst::{
    core::Time,
    ecs::{Entities, Join, Read, System, WriteStorage},
    ui::UiTransform,
};

use crate::entities::map_change_announcement::{MapChangeAnnouncement, MapChangeAnnouncementState};

pub struct MapChangeAnnouncementSystem;

impl<'a> System<'a> for MapChangeAnnouncementSystem {
    type SystemData = (
        WriteStorage<'a, MapChangeAnnouncement>,
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
        let opening_time = 0.4;
        let waiting_time = 3.;
        let closing_time = 0.4;

        let waiting_box_y = -40.;
        let waiting_text_y = -40.;

        for (entity, announcement) in (&entities, &mut announcements).join() {
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
                        continue;
                    }

                    let progress = announcement.elapsed_time / opening_time;
                    set_box_transform_y(waiting_box_y * progress, &mut ui_transforms);
                    set_text_transform_y(waiting_text_y * progress, &mut ui_transforms);
                },
                MapChangeAnnouncementState::Waiting => {
                    if announcement.elapsed_time >= waiting_time {
                        announcement.elapsed_time = 0.;
                        announcement.state = MapChangeAnnouncementState::Closing;
                        continue;
                    }

                    set_box_transform_y(waiting_box_y, &mut ui_transforms);
                    set_text_transform_y(waiting_text_y, &mut ui_transforms);
                },
                MapChangeAnnouncementState::Closing => {
                    if announcement.elapsed_time >= closing_time {
                        entities.delete(announcement.box_entity).expect("Failed to delete announcement box");
                        entities.delete(announcement.text_entity).expect("Failed to delete announcement text");
                        entities.delete(entity).expect("Failed to delete announcement");
                        continue;
                    }

                    let progress = announcement.elapsed_time / closing_time;
                    set_box_transform_y(waiting_box_y * (1. - progress), &mut ui_transforms);
                    set_text_transform_y(waiting_text_y * (1. - progress), &mut ui_transforms);
                },
            }
        }
    }
}
