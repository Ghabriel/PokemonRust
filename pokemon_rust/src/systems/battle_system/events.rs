use amethyst::{
    assets::Handle,
    core::{math::Vector3, Transform},
    ecs::Entity,
    input::{InputEvent, StringBindings},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet},
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    audio::Sound,
    battle::{
        backend::{
            BattleBackend,
            event::{
                ChangeTurn,
                Damage,
                InitialSwitchIn,
                Miss,
                StatChange,
            },
            Team,
        },
        rng::StandardBattleRng,
    },
    constants::{AXIS_SENSITIVITY, BATTLE_CAMERA_POSITION},
    entities::text_box::{advance_text, create_text_box, delete_text_box, TextState},
};

use super::{BattleSystemData, FrontendEvent};

// TODO: move these window-related constants somewhere else
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

// TODO: move to a better place
const SWITCH_IN_ANIMATION_TIME: f32 = 0.5;

const P1_SPRITE_Y: f32 = BATTLE_CAMERA_POSITION.1 - WINDOW_HEIGHT / 4.;
const P2_SPRITE_Y: f32 = BATTLE_CAMERA_POSITION.1 + WINDOW_HEIGHT / 4.;

// Both initial positions should be off-screen to improve the animation
const P1_SPRITE_INITIAL_X: f32 = BATTLE_CAMERA_POSITION.0 - WINDOW_WIDTH / 2. - 128.;
const P2_SPRITE_INITIAL_X: f32 = BATTLE_CAMERA_POSITION.0 + WINDOW_WIDTH / 2. + 128.;

const P1_SPRITE_FINAL_X: f32 = BATTLE_CAMERA_POSITION.0 - WINDOW_WIDTH / 3.;
const P2_SPRITE_FINAL_X: f32 = BATTLE_CAMERA_POSITION.0 + WINDOW_WIDTH / 3.;

fn get_p1_sprite_transform() -> Transform {
    let mut transform = Transform::default();
    transform.set_translation_xyz(P1_SPRITE_INITIAL_X, P1_SPRITE_Y, 0.);
    transform.set_scale(Vector3::new(2., 2., 2.));

    transform
}

fn get_p2_sprite_transform() -> Transform {
    let mut transform = Transform::default();
    transform.set_translation_xyz(P2_SPRITE_INITIAL_X, P2_SPRITE_Y, 0.);
    transform.set_scale(Vector3::new(1.8, 1.8, 1.8));

    transform
}

pub enum InitialSwitchInEvent {
    PendingStart {
        event_data: InitialSwitchIn,
    },
    Started {
        event_data: InitialSwitchIn,
        pokemon_entity: Entity,
        elapsed_time: f32,
    },
}

impl FrontendEvent for InitialSwitchInEvent {
    fn start(
        &mut self,
        backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        if let InitialSwitchInEvent::PendingStart { event_data } = self {
            let BattleSystemData {
                sprite_renders,
                transforms,
                tints,
                entities,
                resources,
                ..
            } = system_data;

            let (sprite_sheet, transform) = if event_data.team == Team::P1 {
                (resources.gen1_back.clone(), get_p1_sprite_transform())
            } else {
                (resources.gen1_front.clone(), get_p2_sprite_transform())
            };

            let pokemon_species = backend.get_species(event_data.pokemon);

            let sprite_render = SpriteRender {
                sprite_sheet,
                sprite_number: pokemon_species.national_number - 1,
            };

            let pokemon_entity = entities
                .build_entity()
                .with(sprite_render, sprite_renders)
                .with(transform, transforms)
                .with(Tint(Srgba::new(1.0, 1.0, 1.0, 0.1)), tints)
                .build();

            let elapsed_time = if event_data.is_already_sent_out {
                SWITCH_IN_ANIMATION_TIME
            } else {
                0.
            };

            *self = InitialSwitchInEvent::Started {
                event_data: event_data.clone(),
                pokemon_entity,
                elapsed_time,
            };
        }
    }

    fn tick(
        &mut self,
        _input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) -> bool {
        if let InitialSwitchInEvent::Started { event_data, pokemon_entity, elapsed_time } = self {
            let BattleSystemData {
                transforms,
                time,
                ..
            } = system_data;

            let transform = transforms
                .get_mut(*pokemon_entity)
                .expect("Failed to retrieve Transform");

            let x = {
                let (initial_x, final_x) = match event_data.team {
                    Team::P1 => (P1_SPRITE_INITIAL_X, P1_SPRITE_FINAL_X),
                    Team::P2 => (P2_SPRITE_INITIAL_X, P2_SPRITE_FINAL_X),
                };
                let progress = (*elapsed_time / SWITCH_IN_ANIMATION_TIME).min(1.);

                initial_x + (final_x - initial_x) * progress
            };
            transform.set_translation_x(x);

            if *elapsed_time >= SWITCH_IN_ANIMATION_TIME {
                true
            } else {
                *elapsed_time += time.delta_seconds();
                false
            }
        } else {
            panic!("Called tick() before start()");
        }
    }
}


pub enum MoveSelectionScreenEvent {
    PendingStart,
    Started {
        selection_arrow_entity: Entity,
        fight_button_entity: Entity,
        run_button_entity: Entity,
        focused_option: i8,
    },
}

const MOVE_SELECTION_ARROW_HEIGHT: f32 = 37.;
const MOVE_SELECTION_BUTTON_SCREEN_MARGIN: f32 = 10.;
const MOVE_SELECTION_BUTTON_HEIGHT: f32 = 47.;
const MOVE_SELECTION_NUM_OPTIONS: i8 = 2;

impl MoveSelectionScreenEvent {
    fn create_selection_arrow(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.selection_arrow.clone(),
            "Selection Arrow".to_string(),
            -MOVE_SELECTION_BUTTON_SCREEN_MARGIN - 162.,
            Self::get_selection_arrow_y(0),
            32.,
            MOVE_SELECTION_ARROW_HEIGHT,
            system_data,
        )
    }

    fn create_fight_button(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.fight_button.clone(),
            "Fight Button".to_string(),
            -MOVE_SELECTION_BUTTON_SCREEN_MARGIN,
            2. * MOVE_SELECTION_BUTTON_SCREEN_MARGIN + MOVE_SELECTION_BUTTON_HEIGHT,
            160.,
            MOVE_SELECTION_BUTTON_HEIGHT,
            system_data,
        )
    }

    fn create_run_button(system_data: &mut BattleSystemData) -> Entity {
        Self::create_move_selection_button(
            system_data.resources.run_button.clone(),
            "Run Button".to_string(),
            -MOVE_SELECTION_BUTTON_SCREEN_MARGIN,
            MOVE_SELECTION_BUTTON_SCREEN_MARGIN,
            161.,
            MOVE_SELECTION_BUTTON_HEIGHT,
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
        let height_difference = MOVE_SELECTION_BUTTON_HEIGHT - MOVE_SELECTION_ARROW_HEIGHT;
        let inverted_option: f32 = (MOVE_SELECTION_NUM_OPTIONS - 1 - focused_option).into();

        (MOVE_SELECTION_BUTTON_SCREEN_MARGIN + MOVE_SELECTION_BUTTON_HEIGHT) * inverted_option
        + MOVE_SELECTION_BUTTON_SCREEN_MARGIN
        + height_difference / 2.
    }
}

impl FrontendEvent for MoveSelectionScreenEvent {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        let selection_arrow_entity = Self::create_selection_arrow(system_data);
        let fight_button_entity = Self::create_fight_button(system_data);
        let run_button_entity = Self::create_run_button(system_data);

        *self = MoveSelectionScreenEvent::Started {
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
                    InputEvent::ActionPressed(action) => {
                        println!("Action: {}", action);
                        sound_kit.play_sound(Sound::SelectOption);
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
                            (*focused_option + MOVE_SELECTION_NUM_OPTIONS + offset)
                            % MOVE_SELECTION_NUM_OPTIONS;

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


pub enum TextEvent {
    PendingStart {
        full_text: String,
    },
    Started {
        text_box_entity: Entity,
    },
}

impl FrontendEvent for TextEvent {
    fn start(
        &mut self,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) {
        if let TextEvent::PendingStart { full_text } = self {
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
                full_text.clone(),
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

            *self = TextEvent::Started { text_box_entity };
        }
    }

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) -> bool {
        if let TextEvent::Started { text_box_entity } = self {
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
                true
            } else {
                false
            }
        } else {
            panic!("Called tick() before start()");
        }
    }
}
