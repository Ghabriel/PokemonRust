use amethyst::{
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
};

use crate::systems::{AudioSystem, BattleSystem};

use std::ops::Deref;

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
            .with(BattleSystem::default(), "battle_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        Trans::None
    }
}
