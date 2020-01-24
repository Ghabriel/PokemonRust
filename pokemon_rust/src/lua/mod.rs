mod polymorphic_container;
mod utils;

use amethyst::{
    ecs::{World, WorldExt},
    utils::application_root_dir,
};

use crate::{
    entities::map::MapCoordinates,
    events::{ChainedEvents, EventQueue, TextEvent, WarpEvent},
};

use rlua::{Context, Error, Function, Lua};

use self::{
    polymorphic_container::PolymorphicContainer,
    utils::remove_event,
};

use std::{
    cell::RefCell,
    fs::read_to_string,
    ops::{Deref, DerefMut},
};

struct ExecutionContext<'a> {
    lua_variables: PolymorphicContainer,
    world: &'a mut World,
}

impl Deref for ExecutionContext<'_> {
    type Target = PolymorphicContainer;

    fn deref(&self) -> &PolymorphicContainer {
        &self.lua_variables
    }
}

impl DerefMut for ExecutionContext<'_> {
    fn deref_mut(&mut self) -> &mut PolymorphicContainer {
        &mut self.lua_variables
    }
}

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
        ($globals:ident, $scope:ident, $context:ident)
        $($target_name:ident: $original_name:ident($($param_name:ident: $param_type:ty),*)),*
    ) => {
        $(
            #[allow(unused_parens)]
            let $target_name = $scope.create_function_mut(|_, ($( $param_name ),*): ($( $param_type ),*)| {
                Ok(
                    $original_name(&mut $context.borrow_mut(), $( $param_name ),*)
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
    let execution_context = RefCell::new(ExecutionContext {
        world,
        lua_variables: PolymorphicContainer::default(),
    });

    lua.context(|context| {
        context.scope(|scope| {
            let globals = context.globals();

            native_functions!(
                (globals, scope, execution_context)
                rust_create_chained_event: create_chained_event(),
                rust_create_text_event: create_text_event(text: String),
                rust_create_warp_event: create_warp_event(map: String, x: u32, y: u32),
                rust_add_event: add_event(chain_key: usize, new_event: usize),
                rust_dispatch_event: dispatch_event(key: usize)
            );

            callback(&context)
        })
    })
}

fn create_chained_event(context: &mut ExecutionContext) -> usize {
    let event = ChainedEvents::default();

    context.store(event)
}

fn create_text_event(context: &mut ExecutionContext, text: String) -> usize {
    let event = TextEvent::new(text, context.world);

    context.store(event)
}

fn create_warp_event(context: &mut ExecutionContext, map: String, x: u32, y: u32) -> usize {
    let event = WarpEvent::new(map, MapCoordinates::new(x, y));

    context.store(event)
}

fn add_event(context: &mut ExecutionContext, chain_key: usize, new_event: usize) {
    let mut chain = context.remove::<ChainedEvents>(chain_key);

    chain.add_event(remove_event(context, new_event));

    context.store_at(chain_key, chain);
}

fn dispatch_event(context: &mut ExecutionContext, key: usize) {
    let event = remove_event(context, key);

    context.world.write_resource::<EventQueue>().push_boxed(event);
}