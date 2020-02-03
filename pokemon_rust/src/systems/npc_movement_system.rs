use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{
        Entities,
        Join,
        Read,
        System,
        WriteExpect,
        WriteStorage,
    },
};

use crate::{
    common::{Direction, get_direction_offset},
    entities::{
        AnimationTable,
        CharacterAnimation,
        character::{Character, StepKind},
        npc::{Npc, NpcAction, NpcAnimation, NpcMovement},
    },
    map::{CoordinateSystem, MapHandler},
};

#[derive(Default)]
pub struct NpcMovementSystem;

impl<'a> System<'a> for NpcMovementSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        WriteStorage<'a, NpcMovement>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationTable<CharacterAnimation>>,
        WriteExpect<'a, MapHandler>,
        Entities<'a>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut characters,
        mut npc_movements,
        mut transforms,
        mut animation_tables,
        mut map,
        entities,
        time,
    ): Self::SystemData) {
        let mut static_npcs = Vec::new();

        for (entity, character, movement_data, transform, animation_table) in (
            &entities,
            &mut characters,
            &mut npc_movements,
            &mut transforms,
            &mut animation_tables,
        ).join() {
            // TODO: extract velocity to constant or use GameConfig::player_walking_speed
            let velocity = 160.;

            let delta_seconds = time.delta_seconds();

            if !movement_data.started {
                let new_animation = get_new_animation(&NpcAction::Moving, &character.facing_direction);
                animation_table.change_animation(new_animation.into());

                if movement_data.step_kind == StepKind::Right {
                    animation_table.skip_to_frame_index(2);
                }

                map.mark_tile_as_solid(&movement_data.to);
                movement_data.started = true;
            }

            if movement_data.estimated_time <= delta_seconds {
                transform.set_translation(Vector3::new(
                    movement_data.to.position.x(),
                    movement_data.to.position.y(),
                    0.,
                ));

                let new_animation = get_new_animation(&NpcAction::Idle, &character.facing_direction);
                animation_table.change_animation(new_animation.into());

                character.next_step.invert();

                map.remove_solid_mark(&movement_data.from);
                static_npcs.push(entity);
                continue;
            }

            movement_data.estimated_time -= delta_seconds;

            let (offset_x, offset_y) = get_direction_offset::<f32>(&character.facing_direction);
            transform.prepend_translation_x(offset_x * velocity * time.delta_seconds());
            transform.prepend_translation_y(offset_y * velocity * time.delta_seconds());
        }

        for entity in static_npcs {
            npc_movements.remove(entity);
        }
    }
}

pub fn get_new_animation(action: &NpcAction, direction: &Direction) -> NpcAnimation {
    match (action, direction) {
        (NpcAction::Idle, Direction::Up) => NpcAnimation::IdleUp,
        (NpcAction::Idle, Direction::Down) => NpcAnimation::IdleDown,
        (NpcAction::Idle, Direction::Left) => NpcAnimation::IdleLeft,
        (NpcAction::Idle, Direction::Right) => NpcAnimation::IdleRight,
        (NpcAction::Moving, Direction::Up) => NpcAnimation::WalkUp,
        (NpcAction::Moving, Direction::Down) => NpcAnimation::WalkDown,
        (NpcAction::Moving, Direction::Left) => NpcAnimation::WalkLeft,
        (NpcAction::Moving, Direction::Right) => NpcAnimation::WalkRight,
    }
}
