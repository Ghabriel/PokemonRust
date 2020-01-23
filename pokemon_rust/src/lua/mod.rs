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

fn run_with_native_functions<F, R>(world: &mut World, lua: &Lua, callback: F) -> Result<R, Error>
where
    F: FnOnce(&Context) -> Result<R, Error>,
{
    let world = RefCell::new(world);

    lua.context(|context| {
        context.scope(|scope| {
            let rust_add_text_event = scope.create_function_mut(|_, text: String| {
                add_text_event(&mut world.borrow_mut(), text);
                Ok(())
            })?;

            let rust_add_warp_event = scope.create_function_mut(|_, (map, x, y): (String, u32, u32)| {
                add_warp_event(&mut world.borrow_mut(), map, x, y);
                Ok(())
            })?;

            let globals = context.globals();
            globals.set("rust_add_text_event", rust_add_text_event)?;
            globals.set("rust_add_warp_event", rust_add_warp_event)?;

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
