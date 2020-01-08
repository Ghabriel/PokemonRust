use amethyst::{
    ecs::{Component, DenseVecStorage},
};

#[derive(Clone, Debug)]
pub struct TextEvent {
    pub text: String,
}

impl TextEvent {
    pub fn new<T>(text: T) -> TextEvent
    where
        T: Into<String>
    {
        TextEvent {
            text: text.into()
        }
    }
}
