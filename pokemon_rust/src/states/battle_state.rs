use amethyst::{
    audio::output::init_output,
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
};

use crate::{
    entities::character::CharacterId,
    systems::{
        AudioSystem,
    },
};

use std::ops::Deref;

/// The state that the game reaches when the player enters a battle.
/// This state makes some assumptions about the state of the World:
///   * An entity representing the battle conditions exists with the following
///     components:
///     * `BattleType`
///     * `OpponentKind`
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
        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(AudioSystem::default(), "audio_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        init_output(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(world);
        }

        Trans::None
    }
}
