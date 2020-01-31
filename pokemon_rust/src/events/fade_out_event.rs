//! Hides the contents of the screen with an animation.
//! Affected by [`GameConfig::fade_duration`](../config/struct.GameConfig.html#structfield.fade_duration).

use amethyst::{
    core::Time,
    ecs::{
        Component,
        Entity,
        HashMapStorage,
        Read,
        ReadExpect,
        SystemData,
        world::Builder,
        World,
        WorldExt,
        WriteStorage,
    },
    renderer::SpriteRender,
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    common::CommonResources,
    config::GameConfig,
};

use super::{GameEvent, ShouldDisableInput};

/// A marker component to allow easy retrieval of the fade-related entities.
pub struct Fade {
    pub id: u8,
}

impl Component for Fade {
    type Storage = HashMapStorage<Self>;
}

#[derive(Default)]
pub struct FadeOutEvent {
    top_fade: Option<Entity>,
    bottom_fade: Option<Entity>,
    elapsed_time: f32,
    completed: bool,
}

impl GameEvent for FadeOutEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        self.top_fade = Some(
            initialise_fade_entity(world, Anchor::TopMiddle, Anchor::TopMiddle, 0)
        );

        self.bottom_fade = Some(
            initialise_fade_entity(world, Anchor::BottomMiddle, Anchor::BottomMiddle, 1)
        );

        ShouldDisableInput(true)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let (mut ui_transforms, game_config, time) = <(
            WriteStorage<UiTransform>,
            ReadExpect<GameConfig>,
            Read<Time>,
        )>::fetch(world);

        let fade_duration = game_config.fade_duration;

        self.elapsed_time += time.delta_seconds();

        ui_transforms
            .get_mut(*self.top_fade.as_mut().unwrap())
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (self.elapsed_time / fade_duration);

        ui_transforms
            .get_mut(*self.bottom_fade.as_mut().unwrap())
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (self.elapsed_time / fade_duration);

        self.completed = self.elapsed_time >= fade_duration;
    }

    fn is_complete(&self) -> bool {
        self.completed
    }
}

fn initialise_fade_entity(world: &mut World, anchor: Anchor, pivot: Anchor, id: u8) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: world.read_resource::<CommonResources>().black.clone(),
        sprite_number: 0,
    };

    let ui_transform = UiTransform::new(
        "Fade".to_string(), anchor, pivot,
        0., 0., 2., 800., 0.
    );

    world.register::<Fade>();

    world
        .create_entity()
        .with(Fade { id })
        .with(UiImage::Sprite(sprite_render))
        .with(ui_transform)
        .build()
}
