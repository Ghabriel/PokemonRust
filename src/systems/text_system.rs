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
    input::{InputEvent, StringBindings},
    renderer::SpriteRender,
    shrev::EventChannel,
    ui::{Anchor, LineMode, UiImage, UiText, UiTransform},
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
    box_entity: Entity,
    text_entity: Entity,
}

#[derive(Debug, Eq, PartialEq)]
enum TextState {
    Running,
    Closed,
}

pub struct TextSystem {
    text_event_reader: ReaderId<TextEvent>,
    input_event_reader: ReaderId<InputEvent<StringBindings>>,
    text_queue: VecDeque<TextEvent>,
    text_box: Option<TextBox>,
}

impl TextSystem {
    pub fn new(world: &mut World) -> TextSystem {
        TextSystem {
            text_event_reader: world
                .write_resource::<EventChannel<TextEvent>>()
                .register_reader(),
            input_event_reader: world
                .write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
            text_queue: VecDeque::new(),
            text_box: None,
        }
    }

    fn advance_text(
        &mut self,
        pressed_action_key: bool,
        time: &Time,
        ui_texts: &mut WriteStorage<UiText>,
    ) -> TextState {
        if let Some(text_box) = self.text_box.as_mut() {
            let is_showing_full_text = text_box.displayed_text_end == text_box.full_text.len();

            if text_box.awaiting_keypress && pressed_action_key {
                if is_showing_full_text {
                    return TextState::Closed;
                } else {
                    text_box.displayed_text_start = text_box.displayed_text_end;
                    text_box.awaiting_keypress = false;
                }
            }

            if text_box.cooldown >= time.delta_seconds() {
                text_box.cooldown -= time.delta_seconds();
            } else {
                text_box.cooldown = TEXT_DELAY;

                let displayed_length = text_box.displayed_text_end - text_box.displayed_text_start;

                if is_showing_full_text || displayed_length == 150 {
                    text_box.awaiting_keypress = true;
                } else {
                    text_box.displayed_text_end += 1;
                }
            }

            ui_texts
                .get_mut(text_box.text_entity)
                .expect("Failed to retrieve UiText")
                .text = text_box.full_text[
                    text_box.displayed_text_start..text_box.displayed_text_end
                ].to_string();

            TextState::Running
        } else {
            TextState::Closed
        }
    }

    fn close_text_box(&mut self, entities: &Entities) {
        if let Some(text_box) = self.text_box.take() {
            entities.delete(text_box.box_entity).expect("Failed to delete text box");
            entities.delete(text_box.text_entity).expect("Failed to delete text");
        }
    }
}

impl<'a> System<'a> for TextSystem {
    type SystemData = (
        WriteStorage<'a, UiImage>,
        WriteStorage<'a, UiText>,
        WriteStorage<'a, UiTransform>,
        Entities<'a>,
        ReadExpect<'a, Resources>,
        Read<'a, Time>,
        Read<'a, EventChannel<TextEvent>>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
    );

    fn run(&mut self, (
        mut ui_images,
        mut ui_texts,
        mut ui_transforms,
        entities,
        resources,
        time,
        text_event_channel,
        input_event_channel,
    ): Self::SystemData) {
        for event in text_event_channel.read(&mut self.text_event_reader) {
            self.text_queue.push_front(event.clone());
        }

        let mut pressed_action_key = false;
        for event in input_event_channel.read(&mut self.input_event_reader) {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    pressed_action_key = true;
                },
                _ => {},
            }
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
                        box_entity: initialise_box_entity(
                            &entities,
                            &mut ui_images,
                            &mut ui_transforms,
                            &resources,
                        ),
                        text_entity: initialise_text_entity(
                            &entities,
                            &mut ui_texts,
                            &mut ui_transforms,
                            &resources,
                        ),
                    }
                });
        }

        let state = self.advance_text(pressed_action_key, &time, &mut ui_texts);

        if state == TextState::Closed {
            self.close_text_box(&entities);
        }
    }
}

fn initialise_box_entity(
    entities: &Entities,
    // sprite_renders: &mut WriteStorage<SpriteRender>,
    ui_images: &mut WriteStorage<UiImage>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &Resources,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: resources.text_box.clone(),
        sprite_number: 0,
    };

    let ui_transform = UiTransform::new(
        "Text Box".to_string(), Anchor::BottomMiddle, Anchor::BottomMiddle,
        0., 20., 2., 800., 100.
    );

    entities
        .build_entity()
        .with(UiImage::Sprite(sprite_render), ui_images)
        .with(ui_transform, ui_transforms)
        .build()
}

fn initialise_text_entity(
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
        "Text".to_string(), Anchor::BottomMiddle, Anchor::BottomLeft,
        -320., 17., 3., 640., 100.
    );

    entities
        .build_entity()
        .with(ui_text, ui_texts)
        .with(ui_transform, ui_transforms)
        .build()
}
