use amethyst::ecs::WorldExt;

use crate::{
    events::{ChainedEvents, EventQueue, GameEvent, NpcMoveEvent, TextEvent, WarpEvent},
    map::MapCoordinates,
};

use super::ExecutionContext;

pub(super) fn create_chained_event(context: &mut ExecutionContext) -> usize {
    let event = ChainedEvents::default();

    context.store(event)
}

pub(super) fn create_npc_move_event(
    context: &mut ExecutionContext,
    npc_id: usize,
    num_tiles: usize,
) -> usize {
    let event = NpcMoveEvent::new(npc_id, num_tiles);

    context.store(event)
}

pub(super) fn create_text_event(context: &mut ExecutionContext, text: String) -> usize {
    let event = TextEvent::new(text, context.world);

    context.store(event)
}

pub(super) fn create_warp_event(context: &mut ExecutionContext, map: String, x: u32, y: u32) -> usize {
    let event = WarpEvent::new(map, MapCoordinates::new(x, y));

    context.store(event)
}

pub(super) fn add_event(context: &mut ExecutionContext, chain_key: usize, new_event: usize) {
    let mut chain = context.remove::<ChainedEvents>(chain_key);

    chain.add_event(remove_event(context, new_event));

    context.store_at(chain_key, chain);
}

pub(super) fn dispatch_event(context: &mut ExecutionContext, key: usize) {
    let event = remove_event(context, key);

    context.world.write_resource::<EventQueue>().push_boxed(event);
}

fn remove_event(
    context: &mut ExecutionContext,
    key: usize,
) -> Box<dyn GameEvent + Send + Sync> {
    let event = context.remove_boxed(key);

    if event.is::<ChainedEvents>() {
        event.downcast::<ChainedEvents>().unwrap()
    } else if event.is::<NpcMoveEvent>() {
        event.downcast::<NpcMoveEvent>().unwrap()
    } else if event.is::<TextEvent>() {
        event.downcast::<TextEvent>().unwrap()
    } else if event.is::<WarpEvent>() {
        event.downcast::<WarpEvent>().unwrap()
    } else {
        panic!("Invalid event type");
    }
}
