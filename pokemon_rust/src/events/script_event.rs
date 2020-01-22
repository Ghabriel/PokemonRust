use amethyst::{
    ecs::{World, WorldExt},
    utils::application_root_dir,
};

use crate::entities::map::{GameScript, MapHandler, MapId};

use rlua::{Function, Lua};

use std::fs::read_to_string;

use super::{GameEvent, ShouldDisableInput};

thread_local! {
    static LUA: Lua = Lua::new();
}

pub struct ScriptEvent {
    map: MapId,
    script_index: usize,
}

impl ScriptEvent {
    pub fn new(map: MapId, script_index: usize) -> ScriptEvent {
        ScriptEvent {
            map,
            script_index,
        }
    }
}

impl GameEvent for ScriptEvent {
    fn start(&mut self, _world: &mut World) -> ShouldDisableInput {
        // TODO: is this always correct?
        ShouldDisableInput(false)
    }

    fn tick(&mut self, world: &mut World, _disabled_inputs: bool) {
        let game_script = world
            .read_resource::<MapHandler>()
            .get_script(&self.map, self.script_index)
            .clone();

        match game_script {
            GameScript::Native(script) => script(world),
            GameScript::Lua { file, function } => {
                let path = application_root_dir()
                    .unwrap()
                    .join("src")
                    .join("lua")
                    .join(&file);

                LUA.with(|lua| {
                    lua.context(|context| {
                        let content = read_to_string(&path)
                            .expect(&format!("Failed to open lua file {}", file));

                        context.load(&content)
                            .exec()
                            .expect(&format!("Failed to parse lua file {}", file));

                        let globals = context.globals();
                        let callback: Function = globals.get(function.as_str())
                            .expect(&format!("Failed to retrieve lua function {}", function));

                        callback.call::<_, ()>(())
                            .expect(&format!("Failed to call lua function {}", function));
                    });
                })
            },
        }
    }

    fn is_complete(&self) -> bool {
        true
    }
}
