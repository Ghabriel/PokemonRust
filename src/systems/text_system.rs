use amethyst::{
    core::Time,
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
    ui::{Anchor, LineMode, UiText, UiTransform},
};

use crate::{
    entities::{
        resources::Resources,
        text::TextEvent,
    },
};

use std::collections::VecDeque;

pub const TEXT_DELAY: f32 = 0.03;

pub struct TextBox {
    full_text: String,
    displayed_text_start: usize,
    displayed_text_end: usize,
    awaiting_keypress: bool,
    cooldown: f32,
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

    fn advance_text(&mut self, time: &Time, ui_texts: &mut WriteStorage<UiText>) {
        if let Some(text_box) = self.text_box.as_mut() {
            if text_box.cooldown >= time.delta_seconds() {
                text_box.cooldown -= time.delta_seconds();
            } else {
                text_box.cooldown = TEXT_DELAY;

                if text_box.displayed_text_end < text_box.full_text.len() {
                    text_box.displayed_text_end += 1;
                } else {
                    text_box.awaiting_keypress = true;
                }
            }

            ui_texts
                .get_mut(text_box.entity)
                .expect("Failed to retrieve UiText")
                .text = text_box.full_text[
                    text_box.displayed_text_start..text_box.displayed_text_end
                ].to_string();
        }
    }
}

impl<'a> System<'a> for TextSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
        Entities<'a>,
        ReadExpect<'a, Resources>,
        Read<'a, Time>,
        Read<'a, EventChannel<TextEvent>>,
    );

    fn run(&mut self, (
        mut ui_texts,
        mut ui_transforms,
        entities,
        resources,
        time,
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
                        displayed_text_end: 0,
                        awaiting_keypress: false,
                        cooldown: TEXT_DELAY,
                        entity: initialise_text_box_entity(
                            &entities,
                            &mut ui_texts,
                            &mut ui_transforms,
                            &resources,
                        ),
                    }
                });
        }

        self.advance_text(&time, &mut ui_texts);
    }
}

fn initialise_text_box_entity(
    entities: &Entities,
    ui_texts: &mut WriteStorage<UiText>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &Resources,
) -> Entity {
    let mut ui_text = UiText::new(
        resources.font.clone(),
        "".to_string(),
        [1., 1., 1., 1.],
        30.,
    );
    ui_text.line_mode = LineMode::Wrap;
    ui_text.align = Anchor::TopLeft;

    let ui_transform = UiTransform::new(
        "Text Box".to_string(), Anchor::BottomMiddle, Anchor::BottomLeft,
        -320., 0., 0., 640., 100.
    );

    entities
        .build_entity()
        .with(ui_text, ui_texts)
        .with(ui_transform, ui_transforms)
        .build()
}
