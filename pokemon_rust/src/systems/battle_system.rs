//! A system responsible for processing Pokémon battles.

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector3, Time, Transform},
    ecs::{Entities, Entity, Read, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::{palette::Srgba, resources::Tint, SpriteRender, SpriteSheet, Texture},
};

use crate::{
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
    common::{AssetTracker, load_sprite_sheet},
    constants::BATTLE_CAMERA_POSITION,
};

use std::collections::VecDeque;

// TODO: move these window-related constants somewhere else
const WINDOW_WIDTH: f32 = 800.;
const WINDOW_HEIGHT: f32 = 600.;

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
#[derive(Default)]
pub struct BattleSystem {
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
}

impl BattleSystem {
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

    fn tick(&mut self, system_data: <Self as System<'_>>::SystemData) {
        let animation = self.active_animation.as_mut().unwrap();

        match animation {
            Animation::InitialSwitchIn { .. } => self.tick_switch_in(system_data),
            Animation::Damage { event_data } => { },
            Animation::Miss => { },
            Animation::StatChange { event_data, time } => { },
        }
    }
}

impl BattleSystem {
    fn init_switch_in(
        &mut self,
        event_data: InitialSwitchIn,
        system_data: &mut <Self as System<'_>>::SystemData,
    ) {
        let (
            battle,
            sprite_renders,
            transforms,
            tints,
            entities,
            asset_tracker,
            loader,
            sprite_sheet_storage,
            texture_storage,
            time,
        ) = system_data;

        let (image_name, transform) = if event_data.team == Team::P1 {
            ("pokemon/gen1_back.png", get_p1_sprite_transform())
        } else {
            ("pokemon/gen1_front.png", get_p2_sprite_transform())
        };

        let sprite_sheet = load_sprite_sheet(
            &loader,
            &texture_storage,
            &sprite_sheet_storage,
            image_name,
            "pokemon/gen1_sprites.ron",
            asset_tracker.get_progress_counter_mut(),
        );

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

        self.active_animation = Some(Animation::InitialSwitchIn {
            event_data,
            pokemon_entity,
            elapsed_time: 0.,
        });
    }

    fn tick_switch_in(
        &mut self,
        (
            battle,
            mut sprite_renders,
            mut transforms,
            mut tints,
            entities,
            mut asset_tracker,
            loader,
            sprite_sheet_storage,
            texture_storage,
            time,
        ): <Self as System<'_>>::SystemData,
    ) {
        let animation = self.active_animation.as_mut().unwrap();

        if let Animation::InitialSwitchIn { event_data, pokemon_entity, elapsed_time } = animation {
            let mut transform = transforms
                .get_mut(*pokemon_entity)
                .expect("Failed to retrieve Transform");

            // TODO: extract to constant
            let total_time = 2.;

            let progress = (*elapsed_time / total_time).min(1.);
            let x = match event_data.team {
                Team::P1 => {
                    P1_SPRITE_INITIAL_X + (P1_SPRITE_FINAL_X - P1_SPRITE_INITIAL_X) * progress
                },
                Team::P2 => {
                    P2_SPRITE_INITIAL_X + (P2_SPRITE_FINAL_X - P2_SPRITE_INITIAL_X) * progress
                },
            };

            transform.set_translation_x(x);

            if *elapsed_time >= total_time {
                self.finish_animation();
            } else {
                *elapsed_time += time.delta_seconds();
            }
        }
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = (
        WriteExpect<'a, Battle>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Tint>,
        Entities<'a>,
        WriteExpect<'a, AssetTracker>,
        ReadExpect<'a, Loader>,
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, AssetStorage<Texture>>,
        Read<'a, Time>,
    );

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
                    self.init_backend(&system_data.0);
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
