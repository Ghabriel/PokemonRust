use amethyst::{
    ecs::{Entities, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
};

use crate::{
    entities::character::{Character, CharacterMovement},
    events::{EventQueue, ScriptEvent},
    map::{GameScript, GameScriptParameters, MapHandler},
};

pub struct NpcInteractionSystem;

impl<'a> System<'a> for NpcInteractionSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        ReadStorage<'a, CharacterMovement>,
        Entities<'a>,
        ReadExpect<'a, MapHandler>,
        WriteExpect<'a, EventQueue>,
    );

    fn run(&mut self, (mut characters, movements, entities, map, mut event_queue): Self::SystemData) {
        for (entity, character, _) in (&entities, &mut characters, !&movements).join() {
            if character.pending_interaction {
                let character_id = map.get_character_id_by_entity(&entity);
                let map_id = map.get_character_natural_map(character_id);

                event_queue.push(ScriptEvent::from_script(
                    GameScript::Lua {
                        file: format!("assets/maps/{}/scripts.lua", map_id.0),
                        function: "interact_with_npc".to_string(),
                        parameters: Some(GameScriptParameters::TargetCharacter(character_id)),
                    }
                ));

                character.pending_interaction = false;
            }
        }
    }
}
