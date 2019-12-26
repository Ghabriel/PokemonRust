use amethyst::{
    animation::{
        AnimationBundle,
        AnimationCommand,
        AnimationControlSet,
        AnimationSet,
        AnimationSetPrefab,
        EndControl,
        get_animation_set,
    },
    assets::{
        Handle,
        Loader,
        PrefabData,
        PrefabLoader,
        PrefabLoaderSystemDesc,
        ProgressCounter,
        RonFormat,
    },
    core::{ArcThreadPool, bundle::SystemBundle, Transform},
    derive::PrefabData,
    ecs::{
        Dispatcher,
        DispatcherBuilder,
        Entities,
        Entity,
        Join,
        ReadStorage,
        world::Builder,
        World,
        WriteStorage,
    },
    Error,
    prelude::*,
    renderer::{
        Camera,
        ImageFormat,
        sprite::prefab::SpriteScenePrefab,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
    },
};

use crate::{
    entities::player::{Player, initialise_player},
};

use serde::{Deserialize, Serialize};
use std::ops::Deref;

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(400., 300., 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(800., 600.))
        .with(transform)
        .build();
}

pub fn load_sprite_sheet(world: &World) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();

    let texture_handle = loader.load(
        "sprites/player.png",
        ImageFormat::default(),
        (),
        &world.read_resource(),
    );

    loader.load(
        "sprites/player.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource(),
    )
}

#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
enum AnimationId {
    Walk,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
struct MyPrefabData {
    sprite_scene: SpriteScenePrefab,
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

#[derive(Default)]
pub struct OverworldState<'a, 'b> {
    pub dispatcher: Option<Dispatcher<'a, 'b>>,
    pub progress_counter: ProgressCounter,
}

impl SimpleState for OverworldState<'_, '_> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Welcome to Pokémon Rust!");

        let mut dispatcher_builder = DispatcherBuilder::new()
            .with(
                PrefabLoaderSystemDesc::<MyPrefabData>::default().build(data.world),
                "scene_loader",
                &[],
            )
            .with_pool(data.world.read_resource::<ArcThreadPool>().deref().clone());

        AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ).build(data.world, &mut dispatcher_builder)
            .expect("Failed to build AnimationBundle");

        let mut dispatcher = dispatcher_builder.build();
        dispatcher.setup(data.world);
        self.dispatcher = Some(dispatcher);

        let player_prefab = data.world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
            loader.load(
                "sprites/player.ron",
                RonFormat,
                &mut self.progress_counter,
            )
        });
        // Creates new entities with components from MyPrefabData
        data.world.create_entity().with(player_prefab).build();

        // data.world.register::<Player>();
        // let sprite_sheet = load_sprite_sheet(data.world);
        // initialise_player(data.world, sprite_sheet.clone());
        initialise_camera(data.world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(data.world);
        }

        if self.progress_counter.is_complete() {
            data.world.exec(
                |(entities, animation_sets, mut control_sets): (
                    Entities,
                    ReadStorage<AnimationSet<AnimationId, SpriteRender>>,
                    WriteStorage<AnimationControlSet<AnimationId, SpriteRender>>,
                )| {
                    for (entity, animation_set) in (&entities, &animation_sets).join() {
                        let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                        control_set.add_animation(
                            AnimationId::Walk,
                            &animation_set.get(&AnimationId::Walk).unwrap(),
                            EndControl::Loop(None),
                            1.0,
                            AnimationCommand::Start,
                        );
                    }
                },
            );
        }

        Trans::None
    }
}
