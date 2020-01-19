use amethyst::{
    assets::ProgressCounter,
    core::Time,
    ecs::{Entity, Read, SystemData, world::Builder, World, WorldExt, WriteStorage},
    renderer::SpriteRender,
    ui::{Anchor, UiImage, UiTransform},
};

use crate::{
    common::load_full_texture_sprite_sheet,
    entities::resources::Resources,
};

use super::{GameEvent, ShouldDisableInput};

pub const FADE_DURATION: f32 = 0.3;

#[derive(Default)]
pub struct FadeOutEvent {
    top_fade: Option<Entity>,
    bottom_fade: Option<Entity>,
    elapsed_time: f32,
}

impl GameEvent for FadeOutEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput {
        println!("FadeOutEvent::start");

        self.top_fade = Some(
            initialise_fade_entity(world, Anchor::TopMiddle, Anchor::TopMiddle)
        );

        self.bottom_fade = Some(
            initialise_fade_entity(world, Anchor::BottomMiddle, Anchor::BottomMiddle)
        );

        ShouldDisableInput(true)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let (mut ui_transforms, time) = <(
            WriteStorage<UiTransform>,
            Read<Time>,
        )>::fetch(world);

        self.elapsed_time += time.delta_seconds();

        ui_transforms
            .get_mut(*self.top_fade.as_mut().unwrap())
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (self.elapsed_time / FADE_DURATION);

        ui_transforms
            .get_mut(*self.bottom_fade.as_mut().unwrap())
            .expect("Failed to retrieve UiTransform")
            .height = 300. * (self.elapsed_time / FADE_DURATION);
    }

    fn is_complete(&self) -> bool {
        self.elapsed_time >= FADE_DURATION
    }
}

fn initialise_fade_entity(world: &mut World, anchor: Anchor, pivot: Anchor) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: world.read_resource::<Resources>().black.clone(),
        sprite_number: 0,
    };

    let ui_transform = UiTransform::new(
        "Fade".to_string(), anchor, pivot,
        0., 0., 2., 800., 0.
    );

    world
        .create_entity()
        .with(UiImage::Sprite(sprite_render))
        .with(ui_transform)
        .build()
}
