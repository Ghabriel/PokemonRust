use amethyst::{
    ecs::{ReadExpect, ReadStorage, System, WriteExpect},
};

use crate::{
    audio::{Sound, SoundKit},
    entities::character::{CharacterMovement, PendingInteraction},
    events::EventQueue,
    map::{interact_with_npc, MapHandler},
};

pub struct NpcInteractionSystem;

impl<'a> System<'a> for NpcInteractionSystem {
    type SystemData = (
        ReadStorage<'a, CharacterMovement>,
        Option<ReadExpect<'a, PendingInteraction>>,
        ReadExpect<'a, MapHandler>,
        WriteExpect<'a, EventQueue>,
        SoundKit<'a>,
    );

    fn run(&mut self, (
        movements,
        pending_interaction,
        map,
        mut event_queue,
        sound_kit,
    ): Self::SystemData) {
        if let Some(pending_interaction) = pending_interaction {
            let character_id = pending_interaction.character_id;
            let entity = map.get_character_by_id(character_id);

            if !movements.contains(entity) {
                let map_id = map.get_character_natural_map(character_id);

                sound_kit.play_sound(Sound::SelectOption);
                interact_with_npc(character_id, &map_id, &mut event_queue);
            }
        }
    }
}
