use amethyst::{
    core::{ArcThreadPool, Transform},
    ecs::{Dispatcher, DispatcherBuilder, Entity},
    prelude::*,
    renderer::{ActiveCamera, Camera},
};

use crate::{
    audio::AudioSystem,
    battle::frontend::BattleSystem,
    constants::{BATTLE_CAMERA_POSITION, WINDOW_HEIGHT, WINDOW_WIDTH},
};

use std::ops::Deref;

pub fn initialise_camera(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(BATTLE_CAMERA_POSITION.0, BATTLE_CAMERA_POSITION.1, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with(transform)
        .build()
}

/// The state that the game reaches when the player enters a battle.
/// This state makes some assumptions about the state of the World:
///   * A resource representing the battle exists (`Battle`);
///   * The player must have an associated `Party` component with at least one
///     Pokémon able to fight;
///   * If this is a trainer battle, then every participating trainer must also
///     fulfill the previous requirement.
#[derive(Default)]
pub struct BattleState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl SimpleState for BattleState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Entering Battle State");

        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(AudioSystem::default(), "audio_system", &[])
            .with(BattleSystem::new(world), "battle_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        let camera = initialise_camera(world);
        world.write_resource::<ActiveCamera>().entity = Some(camera);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        Trans::None
    }
}
