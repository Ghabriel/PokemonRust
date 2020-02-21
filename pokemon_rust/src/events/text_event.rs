//! Displays a text box. Automatically wraps lines and splits the text in pages
//! if needed. Affected by
//! [`GameConfig::text_delay`](../config/struct.GameConfig.html#structfield.text_delay).

use amethyst::{
    ecs::{
        world::Builder,
        Entities,
        Entity,
        ReadExpect,
        SystemData,
        World,
        WorldExt,
        WriteStorage,
    },
    renderer::SpriteRender,
    ui::{Anchor, LineMode, UiImage, UiText, UiTransform},
};

use crate::{common::CommonResources, entities::text_box::TextBox};

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

            TextBox {
                full_text: self.text.clone(),
                displayed_text_start: 0,
                displayed_text_end: 0,
                awaiting_keypress: false,
                cooldown: 0.,
                box_entity: initialise_box_entity(
                    &entities,
                    &mut ui_images,
                    &mut ui_transforms,
                    &resources,
                ),
                text_entity: initialise_text_entity(
                    &entities,
                    &mut ui_texts,
                    &mut ui_transforms,
                    &resources,
                ),
            }
        };

        world.create_entity().with(text_box).build();
    }

    fn tick(&mut self, _world: &mut World, _disabled_inputs: bool) {}

    fn is_complete(&self, world: &mut World) -> bool {
        world.read_storage::<TextBox>().is_empty()
    }
}

fn initialise_box_entity(
    entities: &Entities,
    ui_images: &mut WriteStorage<UiImage>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &CommonResources,
) -> Entity {
    let sprite_render = SpriteRender {
        sprite_sheet: resources.text_box.clone(),
        sprite_number: 0,
    };

    let ui_transform = UiTransform::new(
        "Text Box".to_string(),
        Anchor::BottomMiddle,
        Anchor::BottomMiddle,
        0.,
        20.,
        2.,
        800.,
        100.,
    );

    entities
        .build_entity()
        .with(UiImage::Sprite(sprite_render), ui_images)
        .with(ui_transform, ui_transforms)
        .build()
}

fn initialise_text_entity(
    entities: &Entities,
    ui_texts: &mut WriteStorage<UiText>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    resources: &CommonResources,
) -> Entity {
    let mut ui_text = UiText::new(
        resources.font.clone(),
        "".to_string(),
        [1., 1., 1., 1.],
        30.,
    );
    ui_text.line_mode = LineMode::Wrap;
    ui_text.align = Anchor::TopLeft;

    let ui_transform = UiTransform::new(
        "Text".to_string(),
        Anchor::BottomMiddle,
        Anchor::BottomLeft,
        -320.,
        17.,
        3.,
        640.,
        100.,
    );

    entities
        .build_entity()
        .with(ui_text, ui_texts)
        .with(ui_transform, ui_transforms)
        .build()
}
