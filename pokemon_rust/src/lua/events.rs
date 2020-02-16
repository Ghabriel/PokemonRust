use amethyst::{
    assets::{AssetStorage, Loader},
    audio::Source,
    ecs::WorldExt,
};

use crate::{
    audio::{AudioFileFormat, Music},
    common::Direction,
    entities::character::CharacterId,
    events::{
        BgmChangeEvent,
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

use super::ExecutionContext;

pub(super) fn create_bgm_change_event(context: &mut ExecutionContext, filename: String) -> usize {
    let format = get_bgm_format(&filename);
    let event = BgmChangeEvent::new(filename, format);

    context.store(event)
}

pub(super) fn preload_bgm(context: &mut ExecutionContext, filename: String) {
    let filename = format!("bgm/{}", filename);
    let format = get_bgm_format(&filename);
    let loader = context.world.read_resource::<Loader>();
    let storage = context.world.read_resource::<AssetStorage<Source>>();

    context.world
        .write_resource::<Music>()
        .preload_bgm(filename, format, &loader, &storage);
}

fn get_bgm_format(filename: &str) -> AudioFileFormat {
    if filename.ends_with(".flac") {
        AudioFileFormat::Flac
    } else if filename.ends_with(".mp3") {
        AudioFileFormat::Mp3
    } else if filename.ends_with(".ogg") {
        AudioFileFormat::Ogg
    } else if filename.ends_with(".wav") {
        AudioFileFormat::Wav
    } else {
        panic!("Invalid filename");
    }
}

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
    character_id: CharacterId,
    num_tiles: usize,
) -> usize {
    let event = CharacterMoveEvent::new(character_id, num_tiles);

    context.store(event)
}

pub(super) fn create_npc_rotate_event(
    context: &mut ExecutionContext,
    character_id: CharacterId,
    direction: Direction,
) -> usize {
    let event = CharacterRotateEvent::new(character_id, direction);

    context.store(event)
}

pub(super) fn create_npc_rotate_towards_player_event(
    context: &mut ExecutionContext,
    character_id: CharacterId,
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

    if event.is::<BgmChangeEvent>() {
        event.downcast::<BgmChangeEvent>().unwrap()
    } else if event.is::<ChainedEvents>() {
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
