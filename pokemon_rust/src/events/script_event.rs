//! Runs a [`GameScript`](../map/map/enum.GameScript.html) from the
//! [script repository](../map/map/struct.Map.html#structfield.script_repository)
//! of a map, given its corresponding [`MapId`](../map/struct.MapId.html) and the
//! index of the script.

use amethyst::ecs::{World, WorldExt};

use crate::{
    lua::run_lua_script,
    map::{GameScript, MapHandler, MapId},
};

use super::{BoxedGameEvent, GameEvent, ShouldDisableInput};

#[derive(Clone)]
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
    fn boxed_clone(&self) -> BoxedGameEvent {
        Box::new(self.clone())
    }

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
            GameScript::Lua { file, function, parameters } => {
                let result = run_lua_script(world, &file, &function, &parameters);

                if let Err(err) = result {
                    eprintln!("An error occurred during the execution of a Lua script.");
                    eprintln!("File: {}", file);
                    eprintln!("Function: {}", function);
                    eprintln!("Error message: {}", err);
                }
            },
        }
    }

    fn is_complete(&self, _world: &mut World) -> bool {
        true
    }
}
