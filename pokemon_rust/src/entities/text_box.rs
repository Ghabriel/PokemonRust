use amethyst::ecs::{Component, DenseVecStorage, Entity};

pub struct TextBox {
    pub full_text: String,
    pub displayed_text_start: usize,
    pub displayed_text_end: usize,
    pub awaiting_keypress: bool,
    pub cooldown: f32,
    pub box_entity: Entity,
    pub text_entity: Entity,
}

impl Component for TextBox {
    type Storage = DenseVecStorage<Self>;
}
