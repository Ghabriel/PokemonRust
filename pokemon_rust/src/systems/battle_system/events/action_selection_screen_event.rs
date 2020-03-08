use amethyst::{
    assets::Handle,
    ecs::Entity,
    input::{InputEvent, StringBindings},
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    audio::Sound,
    battle::{
        backend::BattleBackend,
        rng::StandardBattleRng,
    },
    constants::AXIS_SENSITIVITY,
};

use super::super::{BattleSystemData, FrontendEvent};

const ACTION_SELECTION_ARROW_HEIGHT: f32 = 37.;
const ACTION_SELECTION_BUTTON_SCREEN_MARGIN: f32 = 10.;
const ACTION_SELECTION_BUTTON_HEIGHT: f32 = 47.;
const ACTION_SELECTION_NUM_OPTIONS: i8 = 2;

pub enum ActionSelectionScreenEvent {
    PendingStart,
    Started {
        selection_arrow_entity: Entity,
        fight_button_entity: Entity,
        run_button_entity: Entity,
        focused_option: i8,
    },
}

impl ActionSelectionScreenEvent {
    fn create_selection_arrow(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.selection_arrow.clone(),
            "Selection Arrow".to_string(),
            -ACTION_SELECTION_BUTTON_SCREEN_MARGIN - 162.,
            Self::get_selection_arrow_y(0),
            32.,
            ACTION_SELECTION_ARROW_HEIGHT,
            system_data,
        )
    }

    fn create_fight_button(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.fight_button.clone(),
            "Fight Button".to_string(),
            -ACTION_SELECTION_BUTTON_SCREEN_MARGIN,
            2. * ACTION_SELECTION_BUTTON_SCREEN_MARGIN + ACTION_SELECTION_BUTTON_HEIGHT,
            160.,
            ACTION_SELECTION_BUTTON_HEIGHT,
            system_data,
        )
    }

    fn create_run_button(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.run_button.clone(),
            "Run Button".to_string(),
            -ACTION_SELECTION_BUTTON_SCREEN_MARGIN,
            ACTION_SELECTION_BUTTON_SCREEN_MARGIN,
            161.,
            ACTION_SELECTION_BUTTON_HEIGHT,
            system_data,
        )
    }

    fn create_move_selection_button(
        sprite_sheet: Handle<SpriteSheet>,
        id: String,
        right_margin: f32,
        down_margin: f32,
        width: f32,
        height: f32,
        system_data: &mut BattleSystemData,
    ) -> Entity {
        let BattleSystemData {
            ui_images,
            ui_transforms,
            entities,
            ..
        } = system_data;

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        let ui_transform = UiTransform::new(
            id,
            Anchor::BottomRight,
            Anchor::BottomRight,
            right_margin,
            down_margin,
            2.,
            width,
            height,
        );

        entities
            .build_entity()
            .with(UiImage::Sprite(sprite_render), ui_images)
            .with(ui_transform, ui_transforms)
            .build()
    }

    fn update_selection_arrow(&self, system_data: &mut BattleSystemData) {
        if let Self::Started { selection_arrow_entity, focused_option, .. } = self {
            system_data.ui_transforms
                .get_mut(*selection_arrow_entity)
                .expect("Failed to retrieve UiTransform")
                .local_y = Self::get_selection_arrow_y(*focused_option);
        }
    }

    fn get_selection_arrow_y(focused_option: i8) -> f32 {
        let height_difference = ACTION_SELECTION_BUTTON_HEIGHT - ACTION_SELECTION_ARROW_HEIGHT;
        let inverted_option: f32 = (ACTION_SELECTION_NUM_OPTIONS - 1 - focused_option).into();

        (ACTION_SELECTION_BUTTON_SCREEN_MARGIN + ACTION_SELECTION_BUTTON_HEIGHT) * inverted_option
        + ACTION_SELECTION_BUTTON_SCREEN_MARGIN
        + height_difference / 2.
    }

    fn select_option(&mut self) {
        if let Self::Started { focused_option, .. } = self {
            match *focused_option {
                0 => self.select_fight_option(),
                1 => self.select_run_option(),
                _ => unreachable!(),
            }
        }
    }

    fn select_fight_option(&mut self) {
        // TODO
        println!("Selected option: fight");
    }

    fn select_run_option(&mut self) {
        // TODO
        println!("Selected option: run");
    }
}

impl FrontendEvent for ActionSelectionScreenEvent {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        let selection_arrow_entity = Self::create_selection_arrow(system_data);
        let fight_button_entity = Self::create_fight_button(system_data);
        let run_button_entity = Self::create_run_button(system_data);

        *self = ActionSelectionScreenEvent::Started {
            selection_arrow_entity,
            fight_button_entity,
            run_button_entity,
            focused_option: 0,
        };
    }

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData
    ) -> bool {
        for event in input_events {
            let BattleSystemData { sound_kit, .. } = system_data;

            if let Self::Started { focused_option, .. } = self {
                match event {
                    InputEvent::ActionPressed(action) if action == "action" => {
                        sound_kit.play_sound(Sound::SelectOption);
                        self.select_option();
                    },
                    InputEvent::AxisMoved { axis, value } if axis == "vertical" => {
                        let offset = if value < -AXIS_SENSITIVITY {
                            -1
                        } else if value > AXIS_SENSITIVITY {
                            1
                        } else {
                            return false;
                        };

                        *focused_option =
                            (*focused_option + ACTION_SELECTION_NUM_OPTIONS + offset)
                            % ACTION_SELECTION_NUM_OPTIONS;

                        sound_kit.play_sound(Sound::SelectOption);
                        self.update_selection_arrow(system_data);
                    }
                    _ => {},
                }
            } else {
                panic!("Called tick() before start()");
            }
        }

        false
    }
}
