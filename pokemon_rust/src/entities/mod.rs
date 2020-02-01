use amethyst::{
    animation::{
        Animation,
        AnimationCommand,
        AnimationControlSet,
        ControlState,
        InterpolationFunction,
        Sampler,
        SpriteRenderChannel,
        SpriteRenderPrimitive,
    },
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    ecs::{Read, ReadExpect, World},
    renderer::SpriteRender,
};

use serde::{Deserialize, Serialize};

pub mod npc;
pub mod player;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum CharacterAnimation {
    IdleUp,
    IdleDown,
    IdleLeft,
    IdleRight,
    NpcMoveUp,
    NpcMoveDown,
    NpcMoveLeft,
    NpcMoveRight,
    WalkUp,
    WalkDown,
    WalkLeft,
    WalkRight,
    RunUp,
    RunDown,
    RunLeft,
    RunRight,
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
                    output: output.into_iter()
                        .map(SpriteRenderPrimitive::SpriteIndex)
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

pub fn change_character_animation(
    new_animation: CharacterAnimation,
    control_set: &mut AnimationControlSet<CharacterAnimation, SpriteRender>,
) {
    control_set.animations
        .iter_mut()
        .filter(|(id, _)| *id != new_animation)
        .for_each(|(_, animation)| {
            animation.command = AnimationCommand::Pause;
        });

    let (_, animation) = control_set.animations
        .iter_mut()
        .find(|(id, _)| *id == new_animation)
        .unwrap();
    animation.state = ControlState::Requested;
    animation.command = AnimationCommand::Start;
}
