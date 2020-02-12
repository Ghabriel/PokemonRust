use amethyst::{
    core::Time,
    ecs::{Entities, Join, Read, ReaderId, ReadExpect, System, World, WorldExt, WriteStorage},
    input::{InputEvent, StringBindings},
    shrev::EventChannel,
    ui::UiText,
};

use crate::{
    audio::{Sound, SoundKit},
    config::GameConfig,
    entities::text_box::TextBox,
};

#[derive(Debug, Eq, PartialEq)]
enum TextState {
    Running,
    Closed,
}

pub struct TextSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl TextSystem {
    pub fn new(world: &mut World) -> TextSystem {
        TextSystem {
            event_reader: world.write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        }
    }

    fn advance_text(
        &mut self,
        pressed_action_key: bool,
        text_box: &mut TextBox,
        game_config: &GameConfig,
        time: &Time,
        ui_texts: &mut WriteStorage<UiText>,
    ) -> TextState {
        let full_text_length = text_box.full_text.len();
        // TODO: extract to constant or make this more flexible
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
    }
}

impl<'a> System<'a> for TextSystem {
    type SystemData = (
        WriteStorage<'a, TextBox>,
        WriteStorage<'a, UiText>,
        Entities<'a>,
        ReadExpect<'a, GameConfig>,
        Read<'a, Time>,
        Read<'a, EventChannel<InputEvent<StringBindings>>>,
        SoundKit<'a>,
    );

    fn run(&mut self, (
        mut text_boxes,
        mut ui_texts,
        entities,
        game_config,
        time,
        input_event_channel,
        sound_kit,
    ): Self::SystemData) {
        for (entity, text_box) in (&entities, &mut text_boxes).join() {
            let mut pressed_action_key = false;
            for event in input_event_channel.read(&mut self.event_reader) {
                match event {
                    InputEvent::ActionPressed(action) if action == "action" => {
                        pressed_action_key = true;
                        sound_kit.play_sound(Sound::SelectOption);
                    },
                    _ => {},
                }
            }

            let state = self.advance_text(
                pressed_action_key,
                text_box,
                &game_config,
                &time,
                &mut ui_texts,
            );

            if state == TextState::Closed {
                entities.delete(text_box.box_entity).expect("Failed to delete box");
                entities.delete(text_box.text_entity).expect("Failed to delete text");
                entities.delete(entity).expect("Failed to delete text box");
            }
        }
    }
}
