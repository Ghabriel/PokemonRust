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
        character::{Character, CharacterMovement, MovementType, StepKind},
        npc::NpcAnimation,
        player::PlayerEntity,
    },
    map::{CoordinateSystem, MapHandler},
};

#[derive(Default)]
pub struct NpcMovementSystem;

impl<'a> System<'a> for NpcMovementSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        WriteStorage<'a, CharacterMovement>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationTable<CharacterAnimation>>,
        WriteExpect<'a, PlayerEntity>,
        WriteExpect<'a, MapHandler>,
        Entities<'a>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut characters,
        mut npc_movements,
        mut transforms,
        mut animation_tables,
        player_entity,
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
            if entity == player_entity.0 {
                continue;
            }

            let delta_seconds = time.delta_seconds();

            if !movement_data.started {
                let new_animation = get_moving_animation(&character.facing_direction);
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

                let new_animation = get_idle_animation(&character.facing_direction);
                animation_table.change_animation(new_animation.into());

                character.next_step.invert();

                map.remove_solid_mark(&movement_data.from);
                static_npcs.push(entity);
                continue;
            }

            movement_data.estimated_time -= delta_seconds;

            let (offset_x, offset_y) = get_direction_offset::<f32>(&character.facing_direction);
            let frame_velocity = movement_data.velocity * delta_seconds;
            transform.prepend_translation_x(offset_x * frame_velocity);
            transform.prepend_translation_y(offset_y * frame_velocity);
        }

        for entity in static_npcs {
            npc_movements.remove(entity);
        }
    }
}

pub fn get_moving_animation(direction: &Direction) -> NpcAnimation {
    match direction {
        Direction::Up => NpcAnimation::WalkUp,
        Direction::Down => NpcAnimation::WalkDown,
        Direction::Left => NpcAnimation::WalkLeft,
        Direction::Right => NpcAnimation::WalkRight,
    }
}

pub fn get_idle_animation(direction: &Direction) -> NpcAnimation {
    match direction {
        Direction::Up => NpcAnimation::IdleUp,
        Direction::Down => NpcAnimation::IdleDown,
        Direction::Left => NpcAnimation::IdleLeft,
        Direction::Right => NpcAnimation::IdleRight,
    }
}
