use amethyst::ecs::{World, WorldExt};

use crate::entities::map::{GameScript, MapHandler, MapId};

use super::{GameEvent, ShouldDisableInput};

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

        if let GameScript::Native(script) = game_script {
            script(world);
        }
    }

    fn is_complete(&self) -> bool {
        true
    }
}
