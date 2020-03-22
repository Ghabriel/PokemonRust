//! Displays a text box. Automatically wraps lines and splits the text in pages
//! if needed. Affected by
//! [`GameConfig::text_delay`](../config/struct.GameConfig.html#structfield.text_delay).

use amethyst::{
    ecs::{
        world::Builder,
        Entities,
        ReadExpect,
        SystemData,
        World,
        WorldExt,
        WriteStorage,
    },
    ui::{UiImage, UiText, UiTransform},
};

use crate::{
    common::CommonResources,
    text::{create_text_box, TextBox},
};

use super::{BoxedGameEvent, ExecutionConditions, GameEvent};

#[derive(Clone)]
pub struct TextEvent {
    text: String,
}

impl TextEvent {
    pub fn new(text: impl Into<String>) -> TextEvent {
        TextEvent { text: text.into() }
    }
}

impl GameEvent for TextEvent {
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

    fn get_execution_conditions(&self) -> ExecutionConditions {
        ExecutionConditions {
            requires_disabled_input: true,
            ..ExecutionConditions::default()
        }
    }

    fn start(&mut self, world: &mut World) {
        let text_box = {
            let (mut ui_images, mut ui_texts, mut ui_transforms, entities, resources) =
                <(
                    WriteStorage<UiImage>,
                    WriteStorage<UiText>,
                    WriteStorage<UiTransform>,
                    Entities,
                    ReadExpect<CommonResources>,
                )>::fetch(world);

            create_text_box(
                self.text.clone(),
                &mut ui_images,
                &mut ui_texts,
                &mut ui_transforms,
                &entities,
                &resources,
            )
        };

        world.create_entity().with(text_box).build();
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, world: &mut World) -> bool {
        world.read_storage::<TextBox>().is_empty()
    }
}
