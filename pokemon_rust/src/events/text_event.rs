//! Displays a text box. Automatically wraps lines and splits the text in pages
//! if needed. Affected by
//! [`GameConfig::text_delay`](../config/struct.GameConfig.html#structfield.text_delay).

use amethyst::{
    core::Time,
    ecs::{
        Entities,
        Entity,
        Read,
        ReadExpect,
        ReaderId,
        SystemData,
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
    common::CommonResources,
    config::GameConfig,
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

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

pub struct TextEvent {
    text: String,
    input_event_reader: ReaderId<InputEvent<StringBindings>>,
    text_box: Option<TextBox>,
    finished: bool,
}

impl TextEvent {
    pub fn new<T>(text: T, world: &mut World) -> TextEvent
    where
        T: Into<String>
    {
        TextEvent {
            text: text.into(),
            input_event_reader: world
                .write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
            text_box: None,
            finished: false,
        }
    }

    fn advance_text(
        &mut self,
        pressed_action_key: bool,
        game_config: &GameConfig,
        time: &Time,
        ui_texts: &mut WriteStorage<UiText>,
    ) -> TextState {
        if let Some(text_box) = self.text_box.as_mut() {
            let full_text_length = text_box.full_text.len();
            let maximum_display_length = 150;

            match (pressed_action_key, text_box.awaiting_keypress) {
                (true, true) => {
                    if text_box.displayed_text_end == full_text_length {
                        return TextState::Closed;
                    } else {
                        text_box.displayed_text_start = text_box.displayed_text_end;
                        text_box.awaiting_keypress = false;
                    }
                },
                (true, false) => {
                    text_box.displayed_text_end = full_text_length.min(
                        text_box.displayed_text_start + maximum_display_length
                    );
                },
                (false, false) => {
                    text_box.cooldown += time.delta_seconds();
                    while text_box.cooldown >= game_config.text_delay {
                        text_box.cooldown -= game_config.text_delay;

                        let displayed_length = text_box.displayed_text_end - text_box.displayed_text_start;

                        if text_box.displayed_text_end == full_text_length || displayed_length == maximum_display_length {
                            text_box.awaiting_keypress = true;
                        } else {
                            text_box.displayed_text_end += 1;
                        }
                    }
                }
                _ => {},
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

impl GameEvent for TextEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        // TODO
        unimplemented!();
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: true,
        }
    }

    fn start(&mut self, world: &mut World) {
        let (
            mut ui_images,
            mut ui_texts,
            mut ui_transforms,
            entities,
            resources,
        ) = <(
            WriteStorage<UiImage>,
            WriteStorage<UiText>,
            WriteStorage<UiTransform>,
            Entities,
            ReadExpect<CommonResources>,
        )>::fetch(world);

        self.text_box = Some(TextBox {
            full_text: self.text.clone(),
            displayed_text_start: 0,
            displayed_text_end: 0,
            awaiting_keypress: false,
            cooldown: 0.,
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
        });
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let (
            mut ui_texts,
            entities,
            game_config,
            time,
            input_event_channel,
        ) = <(
            WriteStorage<UiText>,
            Entities,
            ReadExpect<GameConfig>,
            Read<Time>,
            Read<EventChannel<InputEvent<StringBindings>>>,
        )>::fetch(world);

        let mut pressed_action_key = false;
        for event in input_event_channel.read(&mut self.input_event_reader) {
            match event {
                InputEvent::ActionPressed(action) if action == "action" => {
                    pressed_action_key = true;
                },
                _ => {},
            }
        }

        let state = self.advance_text(pressed_action_key, &game_config, &time, &mut ui_texts);

        if state == TextState::Closed {
            self.close_text_box(&entities);
            self.finished = true;
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        self.finished
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
    resources: &CommonResources,
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
