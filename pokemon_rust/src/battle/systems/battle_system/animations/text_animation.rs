use amethyst::{
    ecs::Entity,
    input::{InputEvent, StringBindings},
};

use crate::{
    audio::Sound,
    battle::{
        backend::BattleBackend,
        rng::StandardBattleRng,
    },
    text::{advance_text, create_text_box, delete_text_box, TextState},
};

use super::super::{BattleSystemData, FrontendAnimation, TickResult};

pub enum TextAnimation {
    PendingStart {
        text: String,
    },
    Started {
        text_box_entity: Entity,
    },
}

impl FrontendAnimation for TextAnimation {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        if let TextAnimation::PendingStart { text } = self {
            let BattleSystemData {
                text_boxes,
                ui_images,
                ui_texts,
                ui_transforms,
                entities,
                resources,
                ..
            } = system_data;

            let text_box = create_text_box(
                text.clone(),
                ui_images,
                ui_texts,
                ui_transforms,
                &entities,
                &resources,
            );

            let text_box_entity = entities
                .build_entity()
                .with(text_box, text_boxes)
                .build();

            *self = TextAnimation::Started { text_box_entity };
        }
    }

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) -> TickResult {
        if let TextAnimation::Started { text_box_entity } = self {
            let BattleSystemData {
                text_boxes,
                ui_texts,
                entities,
                game_config,
                sound_kit,
                time,
                ..
            } = system_data;

            let mut pressed_action_key = false;
            for event in input_events {
                match event {
                    InputEvent::ActionPressed(action) if action == "action" => {
                        pressed_action_key = true;
                        sound_kit.play_sound(Sound::SelectOption);
                    },
                    _ => {},
                }
            }

            let text_box = text_boxes
                .get_mut(*text_box_entity)
                .expect("Failed to retrieve text box");

            let state = advance_text(
                pressed_action_key,
                text_box,
                &game_config,
                &time,
                ui_texts,
            );

            if state == TextState::Closed {
                delete_text_box(*text_box_entity, text_box, &entities);
                TickResult::done()
            } else {
                TickResult::Incomplete
            }
        } else {
            panic!("Called tick() before start()");
        }
    }
}
