//! A system responsible for processing Pokémon battles.

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{math::Vector3, Transform},
    ecs::{Entities, Read, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::{SpriteRender, SpriteSheet, Texture},
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
};

use std::collections::VecDeque;

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
        time: usize,
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

    fn init_animation(&mut self) {
        let event = self.event_queue.pop_front().unwrap();
        println!("{:?}", event);

        match event {
            BattleEvent::InitialSwitchIn(event_data) => {
                self.active_animation = Some(Animation::InitialSwitchIn { event_data, time: 0 });
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

    fn tick_switch_in(
        &mut self,
        (
            battle,
            mut sprite_renders,
            mut transforms,
            entities,
            mut asset_tracker,
            loader,
            sprite_sheet_storage,
            texture_storage,
        ): <Self as System<'_>>::SystemData,
    ) {
        let animation = self.active_animation.as_mut().unwrap();

        if let Animation::InitialSwitchIn { event_data, time } = animation {
            let (image_name, transform) = if event_data.team == Team::P1 {
                let mut transform = Transform::default();
                // TODO: remove magic numbers
                transform.set_translation_xyz(-1200., -1200., 0.);
                transform.set_scale(Vector3::new(2., 2., 2.));

                ("pokemon/gen1_back.png", transform)
            } else {
                let mut transform = Transform::default();
                // TODO: remove magic numbers
                transform.set_translation_xyz(-800., -800., 0.);
                transform.set_scale(Vector3::new(2., 2., 2.));

                ("pokemon/gen1_front.png", transform)
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

            entities
                .build_entity()
                .with(sprite_render, &mut sprite_renders)
                .with(transform, &mut transforms)
                .build();

            self.finish_animation();
        }
    }
}

impl<'a> System<'a> for BattleSystem {
    type SystemData = (
        WriteExpect<'a, Battle>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transform>,
        Entities<'a>,
        WriteExpect<'a, AssetTracker>,
        ReadExpect<'a, Loader>,
        Read<'a, AssetStorage<SpriteSheet>>,
        Read<'a, AssetStorage<Texture>>,
    );

    fn run(&mut self, system_data: Self::SystemData) {
        if self.temp >= 2 {
            return;
        }

        if self.active_animation.is_none() {
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

            self.init_animation();
        }

        self.tick(system_data);
        self.temp += 1;
    }
}
