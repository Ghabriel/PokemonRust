use amethyst::{
    ecs::{World, WorldExt},
    utils::application_root_dir,
};

use crate::{
    entities::map::MapCoordinates,
    events::{EventQueue, TextEvent, WarpEvent},
};

use rlua::{Context, Error, Function, Lua};

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
        ($globals:ident, $scope:ident, $world:ident)
        $($target_name:ident: $original_name:ident($($param_name:ident: $param_type:ty),*)),*
    ) => {
        $(
            #[allow(unused_parens)]
            let $target_name = $scope.create_function_mut(|_, ($( $param_name ),*): ($( $param_type ),*)| {
                $original_name(&mut $world.borrow_mut(), $( $param_name ),*);
                Ok(())
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

    lua.context(|context| {
        context.scope(|scope| {
            let globals = context.globals();

            native_functions!(
                (globals, scope, world)
                rust_add_text_event: add_text_event(text: String),
                rust_add_warp_event: add_warp_event(map: String, x: u32, y: u32)
            );

            callback(&context)
        })
    })
}

fn add_text_event(world: &mut World, text: String) {
    let event = TextEvent::new(text, world);
    world.write_resource::<EventQueue>().push(event);
}

fn add_warp_event(world: &mut World, map: String, x: u32, y: u32) {
    let event = WarpEvent::new(map, MapCoordinates::new(x, y));
    world.write_resource::<EventQueue>().push(event);
}
