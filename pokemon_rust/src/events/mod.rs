//! Contains every possible game event. Events can trigger animations,
//! textboxes, warps and much more.

pub mod chained_events;
pub mod character_move_event;
pub mod character_single_move_event;
pub mod event_executor;
pub mod event_queue;
pub mod fade_in_event;
pub mod fade_out_event;
pub mod map_interaction_event;
pub mod parallel_events;
pub mod repeated_event;
pub mod script_event;
pub mod switch_map_event;
pub mod text_event;
pub mod warp_event;

use amethyst::ecs::World;

pub use self::{
    chained_events::ChainedEvents,
    character_move_event::CharacterMoveEvent,
    character_single_move_event::CharacterSingleMoveEvent,
    event_executor::EventExecutor,
    event_queue::EventQueue,
    fade_out_event::FadeOutEvent,
    fade_in_event::FadeInEvent,
    map_interaction_event::MapInteractionEvent,
    parallel_events::ParallelEvents,
    repeated_event::RepeatedEvent,
    script_event::ScriptEvent,
    switch_map_event::SwitchMapEvent,
    text_event::TextEvent,
    warp_event::WarpEvent,
};

pub struct ShouldDisableInput(pub bool);

pub trait GameEvent {
    fn start(&mut self, world: &mut World) -> ShouldDisableInput;
    fn tick(&mut self, world: &mut World, disabled_inputs: bool);
    fn is_complete(&self, world: &mut World) -> bool;
}
