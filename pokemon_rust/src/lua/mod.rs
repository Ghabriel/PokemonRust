use amethyst::{
    ecs::{World, WorldExt},
    utils::application_root_dir,
};

use crate::events::{EventQueue, TextEvent};

use rlua::{Error, Function, Lua};

use std::fs::read_to_string;

pub fn run_lua_script(
    world: &mut World,
    lua: &Lua,
    file: &str,
    function: &str,
) -> Result<(), Error> {
    lua.context(move |context| {
        context.scope(|scope| {
            let add_text_event = scope.create_function_mut(|_, text: String| {
                let event = TextEvent::new(text, world);
                world.write_resource::<EventQueue>().push(event);

                Ok(())
            })?;

            let globals = context.globals();
            globals.set("rust_add_text_event", add_text_event)?;

            let path = application_root_dir()
                .unwrap()
                .join("lua")
                .join(&file);

            let content = read_to_string(&path)
                .unwrap_or_else(|_| format!("Failed to open lua file {}", file));

            context.load(&content).exec()?;

            let globals = context.globals();
            let callback: Function = globals.get(function)?;

            callback.call::<_, ()>(())?;

            Ok(())
        })
    })
}
