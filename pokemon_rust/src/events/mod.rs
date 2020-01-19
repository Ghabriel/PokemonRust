pub mod chained_events;
pub mod event_executor;
pub mod event_queue;
pub mod fade_out_event;
pub mod parallel_events;
pub mod text_event;
pub mod warp_event;

use amethyst::ecs::World;

pub use self::{
    chained_events::ChainedEvents,
    event_executor::EventExecutor,
    event_queue::EventQueue,
    fade_out_event::FadeOutEvent,
    parallel_events::ParallelEvents,
    text_event::TextEvent,
    warp_event::WarpEvent,
};

pub struct ShouldDisableInput(pub bool);

pub trait GameEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput;
    fn tick(&mut self, world: &mut World, disabled_inputs: bool);
    fn is_complete(&self) -> bool;
}
