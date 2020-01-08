use amethyst::{
    ecs::{
        Entities,
        Entity,
        Read,
        ReadExpect,
        ReaderId,
        System,
        World,
        WorldExt,
        WriteStorage,
    },
    shrev::EventChannel,
    ui::{Anchor, UiText, UiTransform},
};

use crate::{
    entities::{
        resources::Resources,
        text::TextEvent,
    },
};

use std::collections::VecDeque;

pub struct TextBox {
    full_text: String,
    displayed_text_start: usize,
    displayed_text_end: usize,
    awaiting_keypress: bool,
    entity: Entity,
}

pub struct TextSystem {
    event_reader: ReaderId<TextEvent>,
    text_queue: VecDeque<TextEvent>,
    text_box: Option<TextBox>,
}

impl TextSystem {
    pub fn new(world: &mut World) -> TextSystem {
        TextSystem {
            event_reader: world
                .write_resource::<EventChannel<TextEvent>>()
                .register_reader(),
            text_queue: VecDeque::new(),
            text_box: None,
        }
    }

    fn advance_text(&mut self, ui_texts: &mut WriteStorage<UiText>) {
        if let Some(text_box) = self.text_box.as_mut() {
            ui_texts
                .get_mut(text_box.entity)
                .expect("Failed to retrieve UiText")
                .text = text_box.full_text.clone();
        }
    }
}

impl<'a> System<'a> for TextSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
        Entities<'a>,
        ReadExpect<'a, Resources>,
        Read<'a, EventChannel<TextEvent>>,
    );

    fn run(&mut self, (
        mut ui_texts,
        mut ui_transforms,
        entities,
        resources,
        event_channel,
    ): Self::SystemData) {
        for event in event_channel.read(&mut self.event_reader) {
            self.text_queue.push_front(event.clone());
        }

        if self.text_box.is_none() {
            self.text_box = self.text_queue
                .pop_front()
                .map(|event| {
                    TextBox {
                        full_text: event.text,
                        displayed_text_start: 0,
                        displayed_text_end: 5,
                        awaiting_keypress: false,
                        entity: initialise_text_box_entity(
                            &entities,
                            &mut ui_texts,
                            &mut ui_transforms,
                            &resources,
                        ),
                    }
                });
        }

        self.advance_text(&mut ui_texts);
    }
}

fn initialise_text_box_entity(
    entities: &Entities,
    ui_texts: &mut WriteStorage<UiText>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &Resources,
) -> Entity {
    let ui_transform = UiTransform::new(
        "Text Box".to_string(), Anchor::BottomMiddle, Anchor::BottomLeft,
        -320., 70., 0., 640., 70.
    );

    entities
        .build_entity()
        .with(UiText::new(
            resources.font.clone(),
            "".to_string(),
            [1., 1., 1., 1.],
            30.,
        ), ui_texts)
        .with(ui_transform, ui_transforms)
        .build()
}
