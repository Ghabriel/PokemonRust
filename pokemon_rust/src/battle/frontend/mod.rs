//! A system responsible for processing Pokémon battles.

mod animations;

use amethyst::{
    core::{Time, Transform},
    ecs::{
        Entities,
        Read,
        ReadExpect,
        ReaderId,
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
            event::{
                Damage,
                FailedMove,
                Faint,
                InitialSwitchIn,
                Miss,
                StatChange,
                UseMove,
                VolatileStatusCondition,
            },
            rng::StandardBattleRng,
            BattleBackend,
            BattleEvent,
            Flag,
            FrontendEvent,
            StatChangeKind,
            Team,
            TypeEffectiveness,
        },
        types::Battle,
    },
    common::CommonResources,
    config::GameConfig,
    pokemon::{get_all_pokemon_species, get_pokemon_display_name, Stat},
    text::TextBox,
};

use self::animations::{ActionSelectionScreen, InfoCard, InitialSwitchInAnimation, TextAnimation};

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
    backend: Option<BattleBackend>,
    event_queue: VecDeque<BattleEvent>,
    active_animation_sequence: Option<AnimationSequence>,
    p1_info_card: Option<InfoCard>,
    p2_info_card: Option<InfoCard>,
}

struct AnimationSequence {
    animations: VecDeque<Box<dyn FrontendAnimation + Sync + Send>>,
}

enum TickResult {
    Incomplete,
    Completed {
        new_animations: Vec<Box<dyn FrontendAnimation + Sync + Send>>,
        emitted_events: Vec<FrontendEvent>,
    },
}

impl TickResult {
    fn done() -> Self {
        Self::Completed {
            new_animations: Vec::new(),
            emitted_events: Vec::new(),
        }
    }

    fn replace_by(new_animations: Vec<Box<dyn FrontendAnimation + Sync + Send>>) -> Self {
        Self::Completed {
            new_animations,
            emitted_events: Vec::new(),
        }
    }

    fn emit(event: FrontendEvent) -> Self {
        Self::Completed {
            new_animations: Vec::new(),
            emitted_events: vec![event],
        }
    }
}

trait FrontendAnimation {
    fn start(
        &mut self,
        backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    );

    fn tick(
        &mut self,
        input_events: Vec<InputEvent<StringBindings>>,
        backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    ) -> TickResult;
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
            p1_info_card: None,
            p2_info_card: None,
        }
    }

    fn handle_next_backend_event(&mut self, system_data: &mut BattleSystemData<'_>) {
        let event = self.event_queue.pop_front().unwrap();
        println!("{:?}", event);

        match event {
            BattleEvent::InitialSwitchIn(event_data) => {
                self.handle_initial_switch_in(event_data);
            },
            BattleEvent::ChangeTurn(_) => { },
            BattleEvent::UseMove(event_data) => {
                self.handle_use_move(event_data);
            },
            BattleEvent::Damage(event_data) => {
                self.handle_damage(event_data, system_data);
            },
            BattleEvent::Miss(event_data) => {
                self.handle_miss(event_data);
            },
            BattleEvent::StatChange(event_data) => {
                self.handle_stat_change(event_data);
            },
            BattleEvent::VolatileStatusCondition(event_data) => {
                self.handle_volatile_status_condition(event_data);
            },
            BattleEvent::FailedMove(event_data) => {
                self.handle_failed_move(event_data);
            },
            BattleEvent::Faint(event_data) => {
                self.handle_faint(event_data);
            },
        }

        self.start_animation(system_data);
    }

    fn start_animation(&mut self, system_data: &mut BattleSystemData<'_>) {
        let backend = self.backend.as_mut().unwrap();

        self.active_animation_sequence
            .as_mut()
            .and_then(|sequence| sequence.animations.front_mut())
            .iter_mut()
            .for_each(|animation| animation.start(backend, system_data));
    }

    fn tick(&mut self, system_data: &mut BattleSystemData<'_>) {
        if let Some(active_animation_sequence) = self.active_animation_sequence.as_mut() {
            let animation = active_animation_sequence.animations.front_mut().unwrap();

            let input_events = system_data
                .input_event_channel
                .read(&mut self.event_reader)
                .map(Clone::clone)
                .collect();

            let backend = self.backend.as_mut().unwrap();

            let tick_result = animation.tick(input_events, backend, system_data);

            if let TickResult::Completed {
                new_animations,
                emitted_events,
            } = tick_result {
                active_animation_sequence.animations.pop_front();

                if !emitted_events.is_empty() {
                    for event in emitted_events {
                        backend.push_frontend_event(event);
                    }

                    // TODO: replace this by an AI call
                    use crate::battle::backend::FrontendEventKind;
                    backend.push_frontend_event(FrontendEvent {
                        team: Team::P2,
                        event: FrontendEventKind::UseMove(0),
                    });

                    self.event_queue.extend(backend.tick());
                }

                new_animations
                    .into_iter()
                    .rev()
                    .for_each(|event| active_animation_sequence.animations.push_front(event));

                if let Some(animation) = active_animation_sequence.animations.front_mut() {
                    animation.start(backend, system_data);
                } else {
                    self.active_animation_sequence = None;
                }
            }
        }
    }
}

impl BattleSystem {
    fn handle_initial_switch_in(&mut self, event_data: InitialSwitchIn) {
        let mut animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> = Vec::new();

        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.pokemon);

        match event_data.team {
            Team::P1 => {
                let display_name = get_pokemon_display_name(&pokemon, &pokedex);

                animations.push(Box::new(TextAnimation::PendingStart {
                    text: format!("Go! {}!", display_name),
                }));

                animations.push(Box::new(InitialSwitchInAnimation::PendingStart {
                    event_data: event_data.clone(),
                }));
            },
            Team::P2 => {
                let species = pokedex.get_species(&pokemon.species_id).unwrap();

                animations.push(Box::new(InitialSwitchInAnimation::PendingStart {
                    event_data: event_data.clone(),
                }));

                animations.push(Box::new(TextAnimation::PendingStart {
                    text: format!("A wild {} appears!", species.display_name),
                }));
            },
        }

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_use_move(&mut self, event_data: UseMove) {
        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.move_user);
        let display_name = get_pokemon_display_name(&pokemon, &pokedex);

        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(TextAnimation::PendingStart {
                text: format!("{} used {}!", display_name, event_data.move_name),
            })];

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_damage(&mut self, event_data: Damage, system_data: &mut BattleSystemData<'_>) {
        let mut animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> = Vec::new();

        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.target);

        let info_card = match backend.get_pokemon_team(event_data.target) {
            Team::P1 => self.p1_info_card.as_mut().unwrap(),
            Team::P2 => self.p2_info_card.as_mut().unwrap(),
        };

        info_card.damage(event_data.amount, &pokemon, system_data);

        {
            let effectiveness_text = match event_data.effectiveness {
                TypeEffectiveness::Immune => {
                    let display_name = get_pokemon_display_name(&pokemon, &pokedex);

                    Some(format!("It doesn't affect {}...", display_name))
                },
                TypeEffectiveness::BarelyEffective => Some("It's barely effective...".to_string()),
                TypeEffectiveness::NotVeryEffective => {
                    Some("It's not very effective...".to_string())
                },
                TypeEffectiveness::Normal => {
                    // TODO: remove this after health reduction becomes an animation
                    Some(format!("{} damage!", event_data.amount))
                },
                TypeEffectiveness::SuperEffective => Some("It's super effective!".to_string()),
                TypeEffectiveness::ExtremelyEffective => {
                    Some("It's extremely effective!".to_string())
                },
            };

            if let Some(text) = effectiveness_text {
                animations.push(Box::new(TextAnimation::PendingStart { text }));
            }
        }

        if event_data.is_critical_hit {
            animations.push(Box::new(TextAnimation::PendingStart {
                text: "Critical hit!".to_string(),
            }));
        }

        if event_data.is_last_multi_hit_damage {
            if let Some(index) = event_data.multi_hit_index {
                animations.push(Box::new(TextAnimation::PendingStart {
                    text: format!("Hit {} times!", index + 1),
                }));
            }
        }

        if event_data.is_ohko {
            animations.push(Box::new(TextAnimation::PendingStart {
                text: "It's a one-hit KO!".to_string(),
            }));
        }

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_miss(&mut self, event_data: Miss) {
        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.target);
        let display_name = get_pokemon_display_name(&pokemon, &pokedex);

        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(TextAnimation::PendingStart {
                text: format!("But {} avoided the attack!", display_name),
            })];

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_stat_change(&mut self, event_data: StatChange) {
        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.target);
        let display_name = get_pokemon_display_name(&pokemon, &pokedex);

        let stat = match event_data.stat {
            Stat::HP => unreachable!(),
            Stat::Attack => "attack",
            Stat::Defense => "defense",
            Stat::SpecialAttack => "special attack",
            Stat::SpecialDefense => "special defense",
            Stat::Speed => "speed",
            Stat::Accuracy => "accuracy",
            Stat::Evasion => "evasion",
        };

        let text = match event_data.kind {
            StatChangeKind::WontGoAnyLower => {
                format!("{}'s {} won't go any lower!", display_name, stat)
            },
            StatChangeKind::SeverelyFell => format!("{}'s {} severely fell!", display_name, stat),
            StatChangeKind::HarshlyFell => format!("{}'s {} harshly fell!", display_name, stat),
            StatChangeKind::Fell => format!("{}'s {} fell!", display_name, stat),
            StatChangeKind::Rose => format!("{}'s {} rose!", display_name, stat),
            StatChangeKind::SharplyRose => format!("{}'s {} sharply rose!", display_name, stat),
            StatChangeKind::DrasticallyRose => {
                format!("{}'s {} drastically rose!", display_name, stat)
            },
            StatChangeKind::WontGoAnyHigher => {
                format!("{}'s {} won't go any higher!", display_name, stat)
            },
        };

        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(TextAnimation::PendingStart { text })];

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_volatile_status_condition(&mut self, event_data: VolatileStatusCondition) {
        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.target);
        let display_name = get_pokemon_display_name(&pokemon, &pokedex);
        let mut animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> = Vec::new();

        match event_data.added_flag {
            Flag::Confusion => {
                animations.push(Box::new(TextAnimation::PendingStart {
                    text: format!("{} is confused!", display_name),
                }));
            },
            Flag::StatStages(_) => unreachable!(),
        }

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_failed_move(&mut self, event_data: FailedMove) {
        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(TextAnimation::PendingStart {
                text: "But it failed!".to_string(),
            })];

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn handle_faint(&mut self, event_data: Faint) {
        let pokedex = get_all_pokemon_species();
        let backend = self.backend.as_mut().unwrap();
        let pokemon = backend.get_pokemon(event_data.target);
        let display_name = get_pokemon_display_name(&pokemon, &pokedex);

        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(TextAnimation::PendingStart {
                text: format!("{} fainted!", display_name),
            })];

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn push_action_selection_event(&mut self, system_data: &mut BattleSystemData<'_>) {
        let animations: Vec<Box<dyn FrontendAnimation + Sync + Send>> =
            vec![Box::new(ActionSelectionScreen::PendingStart)];

        if self.p1_info_card.is_none() {
            self.init_info_cards(system_data);
        }

        self.active_animation_sequence = Some(AnimationSequence {
            animations: animations.into(),
        });
    }

    fn init_info_cards(&mut self, system_data: &mut BattleSystemData<'_>) {
        let backend = self.backend.as_mut().unwrap();
        let p1 = backend.get_active_pokemon(Team::P1).next().unwrap();
        let p2 = backend.get_active_pokemon(Team::P2).next().unwrap();

        self.p1_info_card = Some(InfoCard::new(p1, Team::P1, system_data));
        self.p2_info_card = Some(InfoCard::new(p2, Team::P2, system_data));
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = BattleSystemData<'a>;

    fn run(&mut self, mut system_data: Self::SystemData) {
        if self.active_animation_sequence.is_none() {
            if self.event_queue.is_empty() {
                match self.backend.as_mut() {
                    Some(_) => {
                        self.push_action_selection_event(&mut system_data);
                        self.start_animation(&mut system_data);
                    },
                    None => {
                        let mut backend = BattleBackend::new(
                            system_data.battle.clone(),
                            Box::new(StandardBattleRng::default()),
                        );

                        self.event_queue.extend(backend.tick());
                        self.backend = Some(backend);
                        self.handle_next_backend_event(&mut system_data);
                    },
                };
            } else {
                self.handle_next_backend_event(&mut system_data);
            }
        }

        self.tick(&mut system_data);
    }
}
