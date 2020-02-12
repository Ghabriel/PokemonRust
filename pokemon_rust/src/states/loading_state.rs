use amethyst::{
    assets::{Loader, ProgressCounter},
    audio::output::init_output,
    core::{ArcThreadPool, math::Vector3, Parent, Transform},
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entity,
        world::Builder,
        World,
        WorldExt,
    },
    prelude::*,
    renderer::Camera,
    ui::TtfFormat,
};

use crate::{
    audio::initialise_audio,
    common::{AssetTracker, CommonResources, load_full_texture_sprite_sheet},
    config::GameConfig,
    entities::character::{initialise_player, PlayerEntity},
    events::EventQueue,
    map::{initialise_map, MapCoordinates},
    states::OverworldState,
    systems::AudioSystem,
};

use std::ops::Deref;

pub fn initialise_camera(world: &mut World, player: Entity) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.0);
    transform.set_scale(Vector3::new(0.5, 0.5, 0.5));

    world
        .create_entity()
        .with(Camera::standard_2d(800., 600.))
        .with(Parent { entity: player })
        .with(transform)
        .build();
}

pub fn initialise_resources(world: &mut World, progress_counter: &mut ProgressCounter) {
    let font = world.read_resource::<Loader>().load(
        "fonts/arial.ttf",
        TtfFormat,
        &mut *progress_counter,
        &world.read_resource(),
    );

    let text_box = load_full_texture_sprite_sheet(
        world,
        "sprites/text_box.png",
        &(800, 100),
        &mut *progress_counter,
    );

    let black = load_full_texture_sprite_sheet(
        world,
        "sprites/black.png",
        &(32, 32),
        &mut *progress_counter,
    );

    world.insert(CommonResources { font, text_box, black });
}

#[derive(Default)]
pub struct LoadingState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub cached_num_loaded_assets: usize,
}

impl SimpleState for LoadingState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Loading game...");

        let world = data.world;

        let mut dispatcher = DispatcherBuilder::new()
            .with(AudioSystem::default(), "audio_system", &[])
            .with_pool(world.read_resource::<ArcThreadPool>().deref().clone())
            .build();

        dispatcher.setup(world);
        self.dispatcher = Some(dispatcher);

        world.insert(EventQueue::default());

        init_output(world);
        initialise_audio(world);

        let (starting_map, starting_position) = {
            let game_config = world.read_resource::<GameConfig>();

            (
                game_config.player_starting_map.clone(),
                MapCoordinates::from_tuple(&game_config.player_starting_position),
            )
        };

        let mut progress_counter = ProgressCounter::new();

        initialise_resources(world, &mut progress_counter);
        initialise_map(world, &starting_map, &mut progress_counter);

        let player = initialise_player(
            world,
            &starting_map,
            starting_position,
            &mut progress_counter,
        );
        initialise_camera(world, player);

        world.insert(AssetTracker::new(progress_counter));
        world.insert(PlayerEntity(player));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(data.world);
        }

        let asset_tracker = data.world.read_resource::<AssetTracker>();
        let progress_counter = asset_tracker.get_progress_counter();
        let num_finished = progress_counter.num_finished();

        if num_finished != self.cached_num_loaded_assets {
            self.cached_num_loaded_assets = num_finished;

            let total = progress_counter.num_assets();
            let percentage = 100 * num_finished / total;
            println!("Loading... {}% ({}/{})", percentage, num_finished, total);
        }

        if progress_counter.is_complete() {
            Trans::Switch(Box::new(OverworldState::default()))
        } else {
            Trans::None
        }
    }
}
