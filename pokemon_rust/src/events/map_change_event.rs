//! Announces a map change, displaying the name of the reached map.

use amethyst::{
    ecs::{Entities, Entity, ReadExpect, SystemData, World, WorldExt, WriteStorage},
    renderer::SpriteRender,
    ui::{Anchor, LineMode, UiImage, UiText, UiTransform},
};

use crate::{
    common::CommonResources,
    entities::map_change_announcement::{
        MapChangeAnnouncement,
        MapChangeAnnouncementQueue,
        MapChangeAnnouncementState,
    },
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct MapChangeEvent {
    map_name: String,
}

impl MapChangeEvent {
    pub fn new(map_name: impl Into<String>) -> MapChangeEvent {
        MapChangeEvent {
            map_name: map_name.into(),
        }
    }
}

impl GameEvent for MapChangeEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: false,
        }
    }

    fn start(&mut self, world: &mut World) {
        let announcement = {
            let (mut ui_images, mut ui_texts, mut ui_transforms, entities, resources) =
                <(
                    WriteStorage<UiImage>,
                    WriteStorage<UiText>,
                    WriteStorage<UiTransform>,
                    Entities,
                    ReadExpect<CommonResources>,
                )>::fetch(world);

            MapChangeAnnouncement {
                elapsed_time: 0.,
                state: MapChangeAnnouncementState::Opening,
                box_entity: initialise_box_entity(
                    &entities,
                    &mut ui_images,
                    &mut ui_transforms,
                    &resources,
                ),
                text_entity: initialise_text_entity(
                    self.map_name.clone(),
                    &entities,
                    &mut ui_texts,
                    &mut ui_transforms,
                    &resources,
                ),
            }
        };

        world
            .write_resource::<MapChangeAnnouncementQueue>()
            .push(announcement);
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, world: &mut World) -> bool {
        world
            .read_resource::<MapChangeAnnouncementQueue>()
            .is_empty()
    }
}

fn initialise_box_entity(
    entities: &Entities,
    ui_images: &mut WriteStorage<UiImage>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &CommonResources,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: resources.text_box.clone(),
        sprite_number: 0,
    };

    let ui_transform = UiTransform::new(
        "Text Box".to_string(),
        Anchor::TopRight,
        Anchor::BottomRight,
        0.,
        0.,
        2.,
        400.,
        40.,
    );

    entities
        .build_entity()
        .with(UiImage::Sprite(sprite_render), ui_images)
        .with(ui_transform, ui_transforms)
        .build()
}

fn initialise_text_entity(
    text: String,
    entities: &Entities,
    ui_texts: &mut WriteStorage<UiText>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &CommonResources,
) -> Entity {
    let mut ui_text = UiText::new(resources.font.clone(), text, [1., 1., 1., 1.], 30.);
    ui_text.line_mode = LineMode::Wrap;
    ui_text.align = Anchor::Middle;

    let ui_transform = UiTransform::new(
        "Announcement Text".to_string(),
        Anchor::TopRight,
        Anchor::BottomMiddle,
        -200.,
        0.,
        3.,
        400.,
        40.,
    );

    entities
        .build_entity()
        .with(ui_text, ui_texts)
        .with(ui_transform, ui_transforms)
        .build()
}
