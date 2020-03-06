//! A system responsible for processing Pokémon battles.

mod events;

use amethyst::{
    core::{Time, Transform},
    ecs::{
        Entities,
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
    renderer::{resources::Tint, SpriteRender},
    shred::ResourceId,
    shrev::EventChannel,
    ui::{UiImage, UiText, UiTransform},
};

use crate::{
    audio::SoundKit,
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
    entities::text_box::TextBox,
};

use self::events::{InitialSwitchInEvent, TextEvent};

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
    active_animation_sequence: Option<AnimationSequence>,
    temp: usize,
}

struct AnimationSequence {
    animations: VecDeque<Box<dyn FrontendEvent + Sync + Send>>,
}

trait FrontendEvent {
    fn start(
        &mut self,
        backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    );

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        backend: &BattleBackend<StandardBattleRng>,
        system_data: &mut BattleSystemData,
    ) -> bool;
}

impl BattleSystem {
    pub fn new(world: &mut World) -> BattleSystem {
        BattleSystem {
            event_reader: world
                .write_resource::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
            backend: None,
            event_queue: VecDeque::new(),
            active_animation_sequence: None,
            temp: 0,
        }
    }

    fn init_backend(&mut self, battle: &Battle) {
        self.backend = Some(BattleBackend::new(
            battle.clone(),
            StandardBattleRng::default(),
        ));
    }

    fn start_animation(&mut self, system_data: &mut <Self as System<'_>>::SystemData) {
        let event = self.event_queue.pop_front().unwrap();
        println!("{:?}", event);

        match event {
            BattleEvent::InitialSwitchIn(event_data) => {
                self.handle_initial_switch_in(event_data);
            },
            BattleEvent::ChangeTurn(_) => { },
            BattleEvent::Damage(_) => {
                // TODO
                // self.active_animation = Some(Animation::Damage { event_data });
            },
            BattleEvent::Miss(_) => {
                // TODO
                // self.active_animation = Some(Animation::Miss);
            },
            BattleEvent::StatChange(_) => {
                // TODO
                // self.active_animation = Some(Animation::StatChange { event_data, time: 0 });
            },
        }

        let backend = self.backend.as_mut().unwrap();

        self.active_animation_sequence
            .as_mut()
            .and_then(|sequence| sequence.animations.front_mut())
            .iter_mut()
            .for_each(|animation| animation.start(backend, system_data));
    }

    fn tick(&mut self, mut system_data: <Self as System<'_>>::SystemData) {
        if let Some(active_animation_sequence) = self.active_animation_sequence.as_mut() {
            let animation = active_animation_sequence.animations.front_mut().unwrap();

            let input_events = system_data
                .input_event_channel
                .read(&mut self.event_reader)
                .map(Clone::clone)
                .collect();

            let backend = self.backend.as_mut().unwrap();

            let completed = animation.tick(
                input_events,
                backend,
                &mut system_data,
            );

            if completed {
                active_animation_sequence.animations.pop_front();

                if let Some(animation) = active_animation_sequence.animations.front_mut() {
                    animation.start(backend, &mut system_data);
                } else {
                    self.active_animation_sequence = None;
                }
            }
        }
    }
}

impl BattleSystem {
    fn handle_initial_switch_in(&mut self, event_data: InitialSwitchIn) {
        let mut animations: Vec<Box<dyn FrontendEvent + Sync + Send>> = Vec::new();

        let backend = self.backend.as_mut().unwrap();
        let species = backend.get_species(event_data.pokemon);

        match event_data.team {
            Team::P1 => {
                let pokemon = backend.get_pokemon(event_data.pokemon);
                let display_name = pokemon.nickname.as_ref().unwrap_or(&species.display_name);

                animations.push(Box::new(TextEvent::PendingStart {
                    full_text: format!("Go! {}!", display_name),
                }));

                animations.push(Box::new(InitialSwitchInEvent::PendingStart {
                    event_data: event_data.clone(),
                }));
            },
            Team::P2 => {
                animations.push(Box::new(InitialSwitchInEvent::PendingStart {
                    event_data: event_data.clone(),
                }));

                animations.push(Box::new(TextEvent::PendingStart {
                    full_text: format!("A wild {} appears!", species.display_name),
                }));
            },
        }

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = BattleSystemData<'a>;

    fn run(&mut self, mut system_data: Self::SystemData) {
        if self.active_animation_sequence.is_none() {
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

            self.start_animation(&mut system_data);
        }

        self.tick(system_data);
    }
}
