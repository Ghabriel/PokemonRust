mod polymorphic_container;

use amethyst::{
    ecs::{World, WorldExt},
    utils::application_root_dir,
};

use crate::{
    entities::map::MapCoordinates,
    events::{ChainedEvents, EventQueue, TextEvent, WarpEvent},
};

use rlua::{Context, Error, Function, Lua};

use self::polymorphic_container::PolymorphicContainer;

use std::{
    cell::RefCell,
    fs::read_to_string,
};

thread_local! {
    static LUA: Lua = Lua::new();
}

pub fn run_lua_script(world: &mut World, file: &str, function: &str) -> Result<(), Error> {
    LUA.with(|lua| {
        run_script(world, &lua, &file, &function)
    })
}

fn run_script(
    world: &mut World,
    lua: &Lua,
    file: &str,
    function: &str,
) -> Result<(), Error> {
    run_with_native_functions(world, lua, |context| {
        let path = application_root_dir()
            .unwrap()
            .join("lua")
            .join(&file);

        let content = read_to_string(&path)
            .expect("Failed to open lua file");

        context.load(&content).exec()?;

        context.globals()
            .get::<_, Function>(function)?
            .call(())?;

        Ok(())
    })
}

macro_rules! native_functions {
    (
        ($globals:ident, $scope:ident, $world:ident, $vars:ident)
        $($target_name:ident: $original_name:ident($($param_name:ident: $param_type:ty),*)),*
    ) => {
        $(
            #[allow(unused_parens)]
            let $target_name = $scope.create_function_mut(|_, ($( $param_name ),*): ($( $param_type ),*)| {
                Ok(
                    $original_name(&mut $world.borrow_mut(), &mut $vars.borrow_mut(), $( $param_name ),*)
                )
            })?;

            $globals.set(stringify!($target_name), $target_name)?;
        )*
    }
}

fn run_with_native_functions<F, R>(world: &mut World, lua: &Lua, callback: F) -> Result<R, Error>
where
    F: FnOnce(&Context) -> Result<R, Error>,
{
    let world = RefCell::new(world);
    let lua_variables = RefCell::new(PolymorphicContainer::default());

    lua.context(|context| {
        context.scope(|scope| {
            let globals = context.globals();

            native_functions!(
                (globals, scope, world, lua_variables)
                // rust_add_text_event: add_text_event(text: String),
                // rust_add_warp_event: add_warp_event(map: String, x: u32, y: u32)
                rust_create_chained_event: create_chained_event(),
                rust_create_text_event: create_text_event(text: String),
                rust_create_warp_event: create_warp_event(map: String, x: u32, y: u32),
                rust_add_text_event: add_text_event(chain_key: usize, new_event: usize),
                rust_add_warp_event: add_warp_event(chain_key: usize, new_event: usize),
                rust_dispatch_event: dispatch_event(chain_key: usize)
            );

            callback(&context)
        })
    })
}

// fn add_text_event(world: &mut World, storage: &mut PolymorphicContainer, text: String) {
//     let event = TextEvent::new(text, world);
//     world.write_resource::<EventQueue>().push(event);
// }

// fn add_warp_event(world: &mut World, storage: &mut PolymorphicContainer, map: String, x: u32, y: u32) {
//     let event = WarpEvent::new(map, MapCoordinates::new(x, y));
//     world.write_resource::<EventQueue>().push(event);
// }

fn create_chained_event(world: &mut World, storage: &mut PolymorphicContainer) -> usize {
    let event = ChainedEvents::default();

    storage.store(event)
}

fn create_text_event(world: &mut World, storage: &mut PolymorphicContainer, text: String) -> usize {
    let event = TextEvent::new(text, world);

    storage.store(event)
}

fn create_warp_event(world: &mut World, storage: &mut PolymorphicContainer, map: String, x: u32, y: u32) -> usize {
    let event = WarpEvent::new(map, MapCoordinates::new(x, y));

    storage.store(event)
}

fn add_text_event(
    world: &mut World,
    storage: &mut PolymorphicContainer,
    chain_key: usize,
    new_event: usize,
) {
    let mut chain = storage.remove::<ChainedEvents>(chain_key).unwrap();
    let new_event = storage.remove::<TextEvent>(new_event).unwrap();

    chain.add_event(new_event);
    storage.store_at(chain_key, chain);
}

fn add_warp_event(
    world: &mut World,
    storage: &mut PolymorphicContainer,
    chain_key: usize,
    new_event: usize,
) {
    let mut chain = storage.remove::<ChainedEvents>(chain_key).unwrap();
    let new_event = storage.remove::<WarpEvent>(new_event).unwrap();

    chain.add_event(new_event);
    storage.store_at(chain_key, chain);
}

fn dispatch_event(
    world: &mut World,
    storage: &mut PolymorphicContainer,
    chain: usize,
) {
    let chain = storage.remove::<ChainedEvents>(chain).unwrap();

    world.write_resource::<EventQueue>().push(*chain);
}
