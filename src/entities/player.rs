use amethyst::{
    animation::{
        Animation,
        AnimationSet,
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
        FlaggedStorage,
        Read,
        ReadExpect,
        world::Builder,
        World,
        WorldExt,
    },
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat},
};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player {
    pub action: PlayerAction,
    pub facing_direction: Direction,
    pub velocity: [f32; 2],
}

impl Component for Player {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}

fn load_sprite_sheet(world: &World, image_name: &str, ron_name: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(image_name, ImageFormat::default(), (), &world.read_resource());

    loader.load(ron_name, SpriteSheetFormat(texture_handle), (), &world.read_resource())
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlayerAction {
    Walk,
    Run,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum PlayerAnimation {
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    RunUp,
    RunDown,
    RunLeft,
    RunRight,
}

pub struct PlayerSpriteSheets {
    pub walking: Handle<SpriteSheet>,
    pub running: Handle<SpriteSheet>,
}

pub fn make_sprite_animation(
    world: &mut World,
    input: Vec<f32>,
    output: Vec<usize>,
    progress_counter: &mut ProgressCounter,
) -> Handle<Animation<SpriteRender>> {
    world.exec(|
        (loader, sampler_storage, animation_storage): (
            ReadExpect<Loader>,
            Read<AssetStorage<Sampler<SpriteRenderPrimitive>>>,
            Read<AssetStorage<Animation<SpriteRender>>>,
        )
    | {
        let samplers = vec![
            (
                0,
                SpriteRenderChannel::SpriteIndex,
                Sampler {
                    input,
                    output: output.iter()
                        .map(|value| SpriteRenderPrimitive::SpriteIndex(*value))
                        .collect(),
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
                        loader.load_from_data(sampler.clone(), &mut *progress_counter, &sampler_storage),
                    )
                })
                .collect(),
        };

        loader.load_from_data(animation, progress_counter, &animation_storage)
    })
}

pub fn initialise_player(world: &mut World) -> Entity {
    let sprite_sheets = PlayerSpriteSheets {
        walking: load_sprite_sheet(world, "sprites/player-walking.png", "sprites/player-walking.ron"),
        running: load_sprite_sheet(world, "sprites/player-running.png", "sprites/player-running.ron"),
    };

    let player = Player {
        action: PlayerAction::Walk,
        facing_direction: Direction::Down,
        velocity: [0., 0.],
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(200., 300., 0.);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheets.walking.clone(),
        sprite_number: 0,
    };

    let mut progress_counter = ProgressCounter::new();
    let mut animation_set = AnimationSet::new();
    animation_set.insert(PlayerAnimation::WalkDown, make_sprite_animation(
        world,
        vec![0.0, 0.25, 0.5, 0.75, 1.0],
        vec![0, 1, 2, 3],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkLeft, make_sprite_animation(
        world,
        vec![0.0, 0.25, 0.5, 0.75, 1.0],
        vec![4, 5, 6, 7],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkRight, make_sprite_animation(
        world,
        vec![0.0, 0.25, 0.5, 0.75, 1.0],
        vec![8, 9, 10, 11],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::WalkUp, make_sprite_animation(
        world,
        vec![0.0, 0.25, 0.5, 0.75, 1.0],
        vec![12, 13, 14, 15],
        &mut progress_counter,
    ));

    animation_set.insert(PlayerAnimation::RunDown, make_sprite_animation(
        world,
        vec![0.0, 0.1, 0.2, 0.3, 0.4],
        vec![0, 1, 2, 3],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunLeft, make_sprite_animation(
        world,
        vec![0.0, 0.1, 0.2, 0.3, 0.4],
        vec![4, 5, 6, 7],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunRight, make_sprite_animation(
        world,
        vec![0.0, 0.1, 0.2, 0.3, 0.4],
        vec![8, 9, 10, 11],
        &mut progress_counter,
    ));
    animation_set.insert(PlayerAnimation::RunUp, make_sprite_animation(
        world,
        vec![0.0, 0.1, 0.2, 0.3, 0.4],
        vec![12, 13, 14, 15],
        &mut progress_counter,
    ));

    world.insert(sprite_sheets);

    world
        .create_entity()
        .with(player)
        .with(transform)
        .with(sprite_render)
        .with(animation_set)
        .build()
}
