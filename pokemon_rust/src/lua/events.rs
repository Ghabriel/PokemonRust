use amethyst::ecs::WorldExt;

use crate::{
    events::{
        ChainedEvents,
        CharacterMoveEvent,
        CharacterRotateEvent,
        CyclicEvent,
        EventQueue,
        GameEvent,
        TextEvent,
        WarpEvent,
    },
    map::MapCoordinates,
};

use super::{ExecutionContext, parse_lua_direction};

pub(super) fn create_chained_event(context: &mut ExecutionContext) -> usize {
    let event = ChainedEvents::default();

    context.store(event)
}

pub(super) fn create_cyclic_event(context: &mut ExecutionContext, event_key: usize) -> usize {
    let event = CyclicEvent::new(remove_event(context, event_key));

    context.store(event)
}

pub(super) fn create_npc_move_event(
    context: &mut ExecutionContext,
    character_id: usize,
    num_tiles: usize,
) -> usize {
    let event = CharacterMoveEvent::new(character_id, num_tiles);

    context.store(event)
}

pub(super) fn create_npc_rotate_event(
    context: &mut ExecutionContext,
    character_id: usize,
    direction: u8,
) -> usize {
    let event = CharacterRotateEvent::new(character_id, parse_lua_direction(direction));

    context.store(event)
}

pub(super) fn create_npc_rotate_towards_player_event(
    context: &mut ExecutionContext,
    character_id: usize,
) -> usize {
    let event = CharacterRotateEvent::towards_player(character_id);

    context.store(event)
}

pub(super) fn create_text_event(context: &mut ExecutionContext, text: String) -> usize {
    let event = TextEvent::new(text);

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
    } else if event.is::<CharacterMoveEvent>() {
        event.downcast::<CharacterMoveEvent>().unwrap()
    } else if event.is::<CharacterRotateEvent>() {
        event.downcast::<CharacterRotateEvent>().unwrap()
    } else if event.is::<CyclicEvent>() {
        event.downcast::<CyclicEvent>().unwrap()
    } else if event.is::<TextEvent>() {
        event.downcast::<TextEvent>().unwrap()
    } else if event.is::<WarpEvent>() {
        event.downcast::<WarpEvent>().unwrap()
    } else {
        panic!("Invalid event type");
    }
}
