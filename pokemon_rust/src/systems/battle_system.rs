//! A system responsible for processing Pokémon battles.

use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{
        Entities,
        Entity,
        Read,
        ReaderId,
        ReadExpect,
        System,
        SystemData,
        World,
        WorldExt,
        WriteExpect,
        WriteStorage,
    },
    input::{InputEvent, StringBindings},
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
    shred::ResourceId,
    shrev::EventChannel,
    ui::{UiImage, UiText, UiTransform},
};

use crate::{
    audio::{Sound, SoundKit},
    battle::{
        backend::{
            BattleBackend,
            BattleEvent,
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
        types::Battle,
    },
    common::CommonResources,
    config::GameConfig,
    constants::BATTLE_CAMERA_POSITION,
    entities::text_box::{advance_text, create_text_box, delete_text_box, TextBox, TextState},
};

use std::collections::VecDeque;

#[derive(SystemData)]
pub struct BattleSystemData<'a> {
    battle: WriteExpect<'a, Battle>,
    sprite_renders: WriteStorage<'a, SpriteRender>,
    text_boxes: WriteStorage<'a, TextBox>,
    transforms: WriteStorage<'a, Transform>,
    tints: WriteStorage<'a, Tint>,
    ui_images: WriteStorage<'a, UiImage>,
    ui_texts: WriteStorage<'a, UiText>,
    ui_transforms: WriteStorage<'a, UiTransform>,
    entities: Entities<'a>,
    resources: ReadExpect<'a, CommonResources>,
    game_config: ReadExpect<'a, GameConfig>,
    input_event_channel: Read<'a, EventChannel<InputEvent<StringBindings>>>,
    sound_kit: SoundKit<'a>,
    time: Read<'a, Time>,
}

// TODO: move these window-related constants somewhere else
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

// TODO: move to a better place
const SWITCH_IN_ANIMATION_TIME: f32 = 1.;

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

/// A system responsible for processing Pokémon battles. Architecturally,
/// battles are split into two layers: one acts like a "frontend" and the other
/// acts like a "backend". This separation allows the battle mechanics
/// themselves to be independent of their visual representation and processing,
/// improving testability and maintainability considerably.
///
/// With that archicture in mind, this system is the "frontend". The frontend
/// is responsible for receiving events from the backend and displaying them
/// to the screen in an intuitive way. It also handles the player's input,
/// sending signals to the backend whenever an action is taken.
pub struct BattleSystem {
    event_reader: ReaderId<InputEvent<StringBindings>>,
    backend: Option<BattleBackend<StandardBattleRng>>,
    event_queue: VecDeque<BattleEvent>,
    active_animation: Option<Animation>,
    temp: usize,
}

enum Animation {
    InitialSwitchIn {
        event_data: InitialSwitchIn,
        pokemon_entity: Entity,
        elapsed_time: f32,
    },
    Damage {
        // TODO: keep track of HP bar animation parameters
        event_data: Damage,
    },
    Miss,
    StatChange {
        event_data: StatChange,
        time: usize,
    },
    Text {
        text_box_entity: Entity,
    },
}

impl BattleSystem {
    pub fn new(world: &mut World) -> BattleSystem {
        BattleSystem {
            event_reader: world
                .write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
            backend: None,
            event_queue: VecDeque::new(),
            active_animation: None,
            temp: 0,
        }
    }

    fn init_backend(&mut self, battle: &Battle) {
        self.backend = Some(BattleBackend::new(
            battle.clone(),
            StandardBattleRng::default(),
        ));
    }

    fn init_animation(&mut self, system_data: &mut <Self as System<'_>>::SystemData) {
        let event = self.event_queue.pop_front().unwrap();
        println!("{:?}", event);

        match event {
            BattleEvent::InitialSwitchIn(event_data) => {
                self.init_switch_in(event_data, system_data);
            },
            BattleEvent::ChangeTurn(_) => self.finish_animation(),
            BattleEvent::Damage(event_data) => {
                self.active_animation = Some(Animation::Damage { event_data });
            },
            BattleEvent::Miss(_) => {
                self.active_animation = Some(Animation::Miss);
            },
            BattleEvent::StatChange(event_data) => {
                self.active_animation = Some(Animation::StatChange { event_data, time: 0 });
            },
        }
    }

    fn finish_animation(&mut self) {
        self.active_animation = None;
    }

    fn tick(&mut self, mut system_data: <Self as System<'_>>::SystemData) {
        let animation = self.active_animation.as_mut().unwrap();

        match animation {
            Animation::InitialSwitchIn { .. } => self.tick_switch_in(&mut system_data),
            Animation::Damage { event_data } => { },
            Animation::Miss => { },
            Animation::StatChange { event_data, time } => { },
            Animation::Text { .. } => self.tick_text(&mut system_data),
        }
    }
}

impl BattleSystem {
    fn init_switch_in(
        &mut self,
        event_data: InitialSwitchIn,
        system_data: &mut BattleSystemData<'_>,
    ) {
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

        let pokemon_species = self.backend.as_ref().unwrap().get_species(event_data.pokemon);

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

        self.active_animation = Some(Animation::InitialSwitchIn {
            event_data,
            pokemon_entity,
            elapsed_time,
        });
    }

    fn tick_switch_in(&mut self, system_data: &mut BattleSystemData<'_>) {
        let BattleSystemData {
            transforms,
            time,
            ..
        } = system_data;

        let animation = self.active_animation.as_mut().unwrap();

        if let Animation::InitialSwitchIn { event_data, pokemon_entity, elapsed_time } = animation {
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
                if event_data.team == Team::P2 {
                    let text = {
                        let species = self
                            .backend
                            .as_mut()
                            .unwrap()
                            .get_species(event_data.pokemon);

                        format!("A wild {} appears!", species.display_name)
                    };

                    self.init_text(text, system_data);
                } else {
                    self.finish_animation();
                }
            } else {
                *elapsed_time += time.delta_seconds();
            }
        }
    }
}

impl BattleSystem {
    fn init_text(
        &mut self,
        text: String,
        system_data: &mut BattleSystemData<'_>,
    ) {
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
            text,
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

        self.active_animation = Some(Animation::Text { text_box_entity });
    }

    fn tick_text(&mut self, system_data: &mut BattleSystemData<'_>) {
        let BattleSystemData {
            text_boxes,
            ui_texts,
            entities,
            game_config,
            input_event_channel,
            sound_kit,
            time,
            ..
        } = system_data;

        let animation = self.active_animation.as_mut().unwrap();

        if let Animation::Text { text_box_entity } = animation {
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
                self.finish_animation();
            }
        }
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = BattleSystemData<'a>;

    fn run(&mut self, mut system_data: Self::SystemData) {
        if self.active_animation.is_none() {
            if self.temp >= 2 {
                println!("Stopped for debugging purposes");
                return;
            }
            self.temp += 1;

            let backend = match self.backend.as_mut() {
                Some(backend) => backend,
                None => {
                    self.init_backend(&system_data.battle);
                    self.backend.as_mut().unwrap()
                },
            };

            if self.event_queue.is_empty() {
                self.event_queue.extend(backend.tick());
            }

            self.init_animation(&mut system_data);
        }

        self.tick(system_data);
    }
}
