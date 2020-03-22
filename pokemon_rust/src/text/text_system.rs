//! A system responsible for animating a text box on the screen.

use amethyst::{
    core::Time,
    ecs::{Entities, Join, Read, ReadExpect, ReaderId, System, World, WorldExt, WriteStorage},
    input::{InputEvent, StringBindings},
    shrev::EventChannel,
    ui::UiText,
};

use crate::{
    audio::{Sound, SoundKit},
    config::GameConfig,
};

use super::{advance_text, delete_text_box, TextBox, TextState};

/// A system responsible for animating a text box on the screen.
pub struct TextSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
}

impl TextSystem {
    pub fn new(world: &mut World) -> TextSystem {
        TextSystem {
            event_reader: world
                .write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        }
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

    fn run(
        &mut self,
        (
        mut text_boxes,
        mut ui_texts,
        entities,
        game_config,
        time,
        input_event_channel,
        sound_kit,
    ): Self::SystemData,
    ) {
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

            let state = advance_text(
                pressed_action_key,
                text_box,
                &game_config,
                &time,
                &mut ui_texts,
            );

            if state == TextState::Closed {
                delete_text_box(entity, text_box, &entities);
            }
        }
    }
}
