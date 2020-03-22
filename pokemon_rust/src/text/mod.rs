//! A TextBox component used for rendering text.

mod text_system;

use amethyst::{
    core::Time,
    ecs::{Component, DenseVecStorage, Entities, Entity, WriteStorage},
    renderer::SpriteRender,
    ui::{Anchor, LineMode, UiImage, UiText, UiTransform},
};

use crate::{common::CommonResources, config::GameConfig};

pub use self::text_system::TextSystem;

/// A component used for rendering text.
pub struct TextBox {
    /// The entire text that should be displayed. Doesn't necessarily fit in a
    /// single page.
    pub full_text: String,
    /// The index of the first character to be displayed in the current page.
    pub displayed_text_start: usize,
    /// The index **after** the last character to be displayed in the current
    /// page. This varies over time (see [TextSystem](../../systems/text_system/struct.TextSystem.html))
    /// to make the text appear progressively.
    pub displayed_text_end: usize,
    /// Whether or not this text box is waiting for a keypress to go on to the
    /// next page (or close if this is the last page).
    pub awaiting_keypress: bool,
    /// Used to measure the elapsed time since the last character was
    /// displayed. This is used to make the text appear progressively in a
    /// constant speed.
    pub cooldown: f32,
    /// The entity corresponding to the box that this text appears in.
    pub box_entity: Entity,
    /// The entity corresponding to the text that displays this struct.
    pub text_entity: Entity,
}

impl Component for TextBox {
    type Storage = DenseVecStorage<Self>;
}

/// Represents the possible states that a text box can be in.
#[derive(Debug, Eq, PartialEq)]
pub enum TextState {
    Running,
    Closed,
}

pub fn create_text_box(
    full_text: String,
    ui_images: &mut WriteStorage<UiImage>,
    ui_texts: &mut WriteStorage<UiText>,
    ui_transforms: &mut WriteStorage<UiTransform>,
    entities: &Entities,
    resources: &CommonResources,
) -> TextBox {
    TextBox {
        full_text,
        displayed_text_start: 0,
        displayed_text_end: 0,
        awaiting_keypress: false,
        cooldown: 0.,
        box_entity: initialise_box_entity(&entities, ui_images, ui_transforms, &resources),
        text_entity: initialise_text_entity(&entities, ui_texts, ui_transforms, &resources),
    }
}

pub fn delete_text_box(entity: Entity, text_box: &mut TextBox, entities: &Entities) {
    entities
        .delete(text_box.box_entity)
        .expect("Failed to delete box");
    entities
        .delete(text_box.text_entity)
        .expect("Failed to delete text");
    entities.delete(entity).expect("Failed to delete text box");
}

pub fn advance_text(
    pressed_action_key: bool,
    text_box: &mut TextBox,
    game_config: &GameConfig,
    time: &Time,
    ui_texts: &mut WriteStorage<UiText>,
) -> TextState {
    let full_text_length = text_box.full_text.len();
    // TODO: extract to constant or make this more flexible
    let maximum_display_length = 150;

    match (pressed_action_key, text_box.awaiting_keypress) {
        (true, true) => {
            if text_box.displayed_text_end == full_text_length {
                return TextState::Closed;
            } else {
                text_box.displayed_text_start = text_box.displayed_text_end;
                text_box.awaiting_keypress = false;
            }
        },
        (true, false) => {
            text_box.displayed_text_end =
                full_text_length.min(text_box.displayed_text_start + maximum_display_length);
        },
        (false, false) => {
            text_box.cooldown += time.delta_seconds();
            while text_box.cooldown >= game_config.text_delay {
                text_box.cooldown -= game_config.text_delay;

                let displayed_length = text_box.displayed_text_end - text_box.displayed_text_start;

                if text_box.displayed_text_end == full_text_length
                    || displayed_length == maximum_display_length
                {
                    text_box.awaiting_keypress = true;
                } else {
                    text_box.displayed_text_end += 1;
                }
            }
        },
        _ => {},
    }

    ui_texts
        .get_mut(text_box.text_entity)
        .expect("Failed to retrieve UiText")
        .text =
        text_box.full_text[text_box.displayed_text_start..text_box.displayed_text_end].to_string();

    TextState::Running
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
