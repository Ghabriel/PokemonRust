use amethyst::{
    core::{math::Vector3, Transform},
    ecs::Entity,
    input::{InputEvent, StringBindings},
    renderer::{palette::Srgba, resources::Tint, SpriteRender},
};

use crate::{
    battle::backend::{
        event::InitialSwitchIn,
        BattleBackend,
        Team,
    },
    constants::{
        ALLY_HEALTH_BAR_HEIGHT,
        BATTLE_CAMERA_POSITION,
        HEALTH_BAR_MARGIN,
        OPPONENT_HEALTH_BAR_HEIGHT,
        WINDOW_HEIGHT,
        WINDOW_WIDTH,
    },
};

use super::super::{BattleSystemData, FrontendAnimation, TickResult};

// TODO: move to a better place
const SWITCH_IN_ANIMATION_TIME: f32 = 0.5;
const POKEMON_SPRITE_HEIGHT: f32 = 64.;
const P1_SPRITE_SCALING: f32 = 2.5;
const P2_SPRITE_SCALING: f32 = 2.2;

const P1_SPRITE_Y: f32 = (BATTLE_CAMERA_POSITION.1 - WINDOW_HEIGHT / 2.)
    + HEALTH_BAR_MARGIN
    + ALLY_HEALTH_BAR_HEIGHT
    + (POKEMON_SPRITE_HEIGHT * P1_SPRITE_SCALING) / 2.
    - 18.; // TODO: investigate why this offset is needed
const P2_SPRITE_Y: f32 = (BATTLE_CAMERA_POSITION.1 + WINDOW_HEIGHT / 2.)
    - HEALTH_BAR_MARGIN
    - OPPONENT_HEALTH_BAR_HEIGHT
    - (POKEMON_SPRITE_HEIGHT * P2_SPRITE_SCALING) / 2.;

// Both initial positions should be off-screen to improve the animation
const P1_SPRITE_INITIAL_X: f32 = BATTLE_CAMERA_POSITION.0 - WINDOW_WIDTH / 2. - 128.;
const P2_SPRITE_INITIAL_X: f32 = BATTLE_CAMERA_POSITION.0 + WINDOW_WIDTH / 2. + 128.;

const P1_SPRITE_FINAL_X: f32 = BATTLE_CAMERA_POSITION.0 - WINDOW_WIDTH / 3.;
const P2_SPRITE_FINAL_X: f32 = BATTLE_CAMERA_POSITION.0 + WINDOW_WIDTH / 3.;

fn get_p1_sprite_transform() -> Transform {
    let mut transform = Transform::default();
    transform.set_translation_xyz(P1_SPRITE_INITIAL_X, P1_SPRITE_Y, 0.);
    transform.set_scale(Vector3::new(
        P1_SPRITE_SCALING,
        P1_SPRITE_SCALING,
        P1_SPRITE_SCALING,
    ));

    transform
}

fn get_p2_sprite_transform() -> Transform {
    let mut transform = Transform::default();
    transform.set_translation_xyz(P2_SPRITE_INITIAL_X, P2_SPRITE_Y, 0.);
    transform.set_scale(Vector3::new(
        P2_SPRITE_SCALING,
        P2_SPRITE_SCALING,
        P2_SPRITE_SCALING,
    ));

    transform
}

pub enum InitialSwitchInAnimation {
    PendingStart {
        event_data: InitialSwitchIn,
    },
    Started {
        event_data: InitialSwitchIn,
        pokemon_entity: Entity,
        elapsed_time: f32,
    },
}

impl FrontendAnimation for InitialSwitchInAnimation {
    fn start(
        &mut self,
        backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    ) {
        if let InitialSwitchInAnimation::PendingStart { event_data } = self {
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

            *self = InitialSwitchInAnimation::Started {
                event_data: event_data.clone(),
                pokemon_entity,
                elapsed_time,
            };
        }
    }

    fn tick(
        &mut self,
        _input_events: Vec<InputEvent<StringBindings>>,
        _backend: &BattleBackend,
        system_data: &mut BattleSystemData,
    ) -> TickResult {
        if let InitialSwitchInAnimation::Started {
            event_data,
            pokemon_entity,
            elapsed_time,
        } = self {
            let BattleSystemData {
                transforms, time, ..
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
                TickResult::done()
            } else {
                *elapsed_time += time.delta_seconds();
                TickResult::Incomplete
            }
        } else {
            panic!("Called tick() before start()");
        }
    }
}
