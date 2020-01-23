use amethyst::ecs::{World, WorldExt};

use crate::{
    entities::map::{GameScript, MapHandler, MapId},
    lua::run_lua_script,
};

use rlua::Lua;

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

    fn tick<'a>(&mut self, world: &'a mut World, _disabled_inputs: bool) {
        let game_script = world
            .read_resource::<MapHandler>()
            .get_script(&self.map, self.script_index)
            .clone();

        match game_script {
            GameScript::Native(script) => script(world),
            GameScript::Lua { file, function } => {
                LUA.with(|lua| {
                    run_lua_script(world, &lua, &file, &function)
                        .expect("Failed to run Lua script");
                })
            },
        }
    }

    fn is_complete(&self) -> bool {
        true
    }
}
