use crate::events::{ChainedEvents, GameEvent, TextEvent, WarpEvent};

use super::ExecutionContext;

pub(super) fn remove_event(
    context: &mut ExecutionContext,
    key: usize,
) -> Box<dyn GameEvent + Send + Sync> {
    let event = context.remove_boxed(key);

    if event.is::<ChainedEvents>() {
        event.downcast::<ChainedEvents>().unwrap()
    } else if event.is::<TextEvent>() {
        event.downcast::<TextEvent>().unwrap()
    } else if event.is::<WarpEvent>() {
        event.downcast::<WarpEvent>().unwrap()
    } else {
        panic!("Invalid event type");
    }
}
