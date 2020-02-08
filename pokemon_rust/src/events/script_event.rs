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
    script: Script,
}

#[derive(Clone)]
enum Script {
    Reference {
        map: MapId,
        script_index: usize,
    },
    Instance(GameScript),
}

impl ScriptEvent {
    pub fn new(map: MapId, script_index: usize) -> ScriptEvent {
        ScriptEvent {
            script: Script::Reference {
                map,
                script_index,
            }
        }
    }

    pub fn from_script(script: GameScript) -> ScriptEvent {
        ScriptEvent {
            script: Script::Instance(script),
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
        let game_script = match &self.script {
            Script::Reference { map, script_index } => world
                .read_resource::<MapHandler>()
                .get_script(&map, *script_index)
                .clone(),
            Script::Instance(script) => script.clone(),
        };

        match game_script {
            GameScript::Native { script, parameters } => script(world, &parameters),
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
