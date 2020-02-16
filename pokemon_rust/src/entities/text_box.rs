//! A TextBox component used for rendering text.

use amethyst::ecs::{Component, DenseVecStorage, Entity};

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
