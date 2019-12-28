use amethyst::{
    animation::{
        Animation,
        AnimationCommand,
        AnimationControlSet,
        AnimationSet,
        ControlState,
        InterpolationFunction,
        Sampler,
        SpriteRenderChannel,
        SpriteRenderPrimitive,
    },
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    core::Transform,
    ecs::{
        Component,
        DenseVecStorage,
        Entity,
        Join,
        Read,
        ReadExpect,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat},
};

use serde::{Deserialize, Serialize};

pub struct Player {
    pub velocity: [f32; 2],
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn load_sprite_sheet(world: &World, image_name: &str, ron_name: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(image_name, ImageFormat::default(), (), &world.read_resource());

    loader.load(ron_name, SpriteSheetFormat(texture_handle), (), &world.read_resource())
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum AnimationId {
    Walk,
    Run,
}

pub struct PlayerSpriteSheets {
    walking: Handle<SpriteSheet>,
    running: Handle<SpriteSheet>,
}

pub fn player_walk(world: &mut World) {
    let sprite_sheets = world.read_resource::<PlayerSpriteSheets>();
    let players = world.read_storage::<Player>();
    let mut sprite_renders = world.write_storage::<SpriteRender>();
    let mut control_sets = world.write_storage::<AnimationControlSet<AnimationId, SpriteRender>>();

    for (_, sprite_render, control_set) in (&players, &mut sprite_renders, &mut control_sets).join() {
        sprite_render.sprite_sheet = sprite_sheets.walking.clone();
        sprite_render.sprite_number = 0;

        control_set.animations
            .iter_mut()
            .filter(|(id, _)| *id != AnimationId::Walk)
            .for_each(|(_, animation)| {
                animation.command = AnimationCommand::Pause;
            });

        let (_, animation) = control_set.animations
            .iter_mut()
            .find(|(id, _)| *id == AnimationId::Walk)
            .unwrap();
        animation.state = ControlState::Requested;
        animation.command = AnimationCommand::Start;
    }
}

pub fn player_run(world: &mut World) {
    let sprite_sheets = world.read_resource::<PlayerSpriteSheets>();
    let players = world.read_storage::<Player>();
    let mut sprite_renders = world.write_storage::<SpriteRender>();
    let mut control_sets = world.write_storage::<AnimationControlSet<AnimationId, SpriteRender>>();

    for (_, sprite_render, control_set) in (&players, &mut sprite_renders, &mut control_sets).join() {
        sprite_render.sprite_sheet = sprite_sheets.running.clone();
        sprite_render.sprite_number = 0;

        control_set.animations
            .iter_mut()
            .filter(|(id, _)| *id != AnimationId::Run)
            .for_each(|(_, animation)| {
                animation.command = AnimationCommand::Pause;
            });

        let (_, animation) = control_set.animations
            .iter_mut()
            .find(|(id, _)| *id == AnimationId::Run)
            .unwrap();
        animation.state = ControlState::Requested;
        animation.command = AnimationCommand::Start;
    }
}

pub fn initialise_player(world: &mut World) -> Entity {
    let sprite_sheets = PlayerSpriteSheets {
        walking: load_sprite_sheet(world, "sprites/player-walking.png", "sprites/player-walking.ron"),
        running: load_sprite_sheet(world, "sprites/player-running.png", "sprites/player-running.ron"),
    };

    let player = Player {
        velocity: [0., 0.],
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(200., 300., 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheets.walking.clone(),
        sprite_number: 0,
    };

    let mut animation_set = AnimationSet::new();
    animation_set.insert(AnimationId::Walk, {
        let mut progress_counter = ProgressCounter::new();

        world.exec(|
            (loader, sampler_storage, animation_storage): (
                ReadExpect<Loader>,
                Read<AssetStorage<Sampler<SpriteRenderPrimitive>>>,
                Read<AssetStorage<Animation<SpriteRender>>>,
            )
        | {
            let samplers: Vec<(usize, SpriteRenderChannel, Sampler<SpriteRenderPrimitive>)> = vec![
                (
                    0,
                    SpriteRenderChannel::SpriteIndex,
                    Sampler {
                        input: vec![0.0, 0.25, 0.5, 0.75, 1.0],
                        output: vec![
                            SpriteRenderPrimitive::SpriteIndex(0),
                            SpriteRenderPrimitive::SpriteIndex(1),
                            SpriteRenderPrimitive::SpriteIndex(2),
                            SpriteRenderPrimitive::SpriteIndex(3),
                        ],
                        function: InterpolationFunction::Step,
                    }
                )
            ];

            let animation = Animation::<SpriteRender> {
                nodes: samplers
                    .iter()
                    .map(|(node_index, channel, sampler)| {
                        (
                            *node_index,
                            channel.clone(),
                            loader.load_from_data(sampler.clone(), &mut progress_counter, &sampler_storage),
                        )
                    })
                    .collect(),
            };

            loader.load_from_data(animation, &mut progress_counter, &animation_storage)
        })
    });
    animation_set.insert(AnimationId::Run, {
        let mut progress_counter = ProgressCounter::new();

        world.exec(|
            (loader, sampler_storage, animation_storage): (
                ReadExpect<Loader>,
                Read<AssetStorage<Sampler<SpriteRenderPrimitive>>>,
                Read<AssetStorage<Animation<SpriteRender>>>,
            )
        | {
            let samplers: Vec<(usize, SpriteRenderChannel, Sampler<SpriteRenderPrimitive>)> = vec![
                (
                    0,
                    SpriteRenderChannel::SpriteIndex,
                    Sampler {
                        input: vec![0.0, 0.25, 0.5, 0.75, 1.0],
                        output: vec![
                            SpriteRenderPrimitive::SpriteIndex(4),
                            SpriteRenderPrimitive::SpriteIndex(5),
                            SpriteRenderPrimitive::SpriteIndex(6),
                            SpriteRenderPrimitive::SpriteIndex(7),
                        ],
                        function: InterpolationFunction::Step,
                    }
                )
            ];

            let animation = Animation::<SpriteRender> {
                nodes: samplers
                    .iter()
                    .map(|(node_index, channel, sampler)| {
                        (
                            *node_index,
                            channel.clone(),
                            loader.load_from_data(sampler.clone(), &mut progress_counter, &sampler_storage),
                        )
                    })
                    .collect(),
            };

            loader.load_from_data(animation, &mut progress_counter, &animation_storage)
        })
    });

    world.insert(sprite_sheets);

    world
        .create_entity()
        .with(player)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build()
}
