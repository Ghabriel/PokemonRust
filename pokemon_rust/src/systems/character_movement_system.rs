use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{
        Entities,
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        Write,
        WriteExpect,
        WriteStorage,
    },
    renderer::SpriteRender,
};

use crate::{
    common::get_direction_offset,
    entities::{
        AnimationTable,
        character::{
            AllowedMovements,
            Character,
            CharacterAnimation,
            CharacterMovement,
            MovementType,
            StepKind,
        },
        player::PlayerEntity,
    },
    events::EventQueue,
    map::{change_tile, CoordinateSystem, MapHandler},
};

pub struct CharacterMovementSystem;

impl<'a> System<'a> for CharacterMovementSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        WriteStorage<'a, CharacterMovement>,
        ReadStorage<'a, AllowedMovements>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationTable<CharacterAnimation>>,
        WriteStorage<'a, SpriteRender>,
        Entities<'a>,
        ReadExpect<'a, PlayerEntity>,
        WriteExpect<'a, MapHandler>,
        Write<'a, EventQueue>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut characters,
        mut movements,
        allowed_movements_storage,
        mut transforms,
        mut animation_tables,
        mut sprite_renders,
        entities,
        player_entity,
        mut map,
        mut event_queue,
        time,
    ): Self::SystemData) {
        let mut static_characters = Vec::new();

        for (entity, character, movement_data, allowed_movements, transform, animation_table, sprite_render) in (
            &entities,
            &mut characters,
            &mut movements,
            &allowed_movements_storage,
            &mut transforms,
            &mut animation_tables,
            &mut sprite_renders,
        ).join() {
            let delta_seconds = time.delta_seconds();
            let is_player = entity == player_entity.0;

            if !movement_data.started {
                if is_player {
                    if map.is_tile_blocked(&movement_data.to) {
                        static_characters.push(entity);
                        continue;
                    }
                }

                let data = allowed_movements
                    .get_movement_data(&movement_data.movement_type)
                    .unwrap();
                sprite_render.sprite_sheet = data.sprite_sheet.clone();

                animation_table.change_animation(CharacterAnimation::Moving(
                    movement_data.movement_type.clone(),
                    character.facing_direction.clone(),
                ));

                if movement_data.step_kind == StepKind::Right {
                    animation_table.skip_to_frame_index(2);
                }

                if !is_player {
                    map.mark_tile_as_solid(&movement_data.to);
                }

                movement_data.started = true;
            }

            if movement_data.estimated_time <= delta_seconds {
                transform.set_translation(Vector3::new(
                    movement_data.to.position.x(),
                    movement_data.to.position.y(),
                    0.,
                ));

                change_tile(
                    &movement_data.from,
                    &movement_data.to,
                    &player_entity,
                    &mut map,
                    &mut event_queue,
                );

                if movement_data.movement_type == MovementType::Run {
                    let data = allowed_movements
                        .get_movement_data(&MovementType::Walk)
                        .unwrap();

                    sprite_render.sprite_sheet = data.sprite_sheet.clone();
                }

                animation_table.change_animation(CharacterAnimation::Idle(
                    character.facing_direction.clone(),
                ));

                if !is_player {
                    map.remove_solid_mark(&movement_data.from);
                }

                character.next_step.invert();

                static_characters.push(entity);
                continue;
            }

            movement_data.estimated_time -= delta_seconds;

            let (offset_x, offset_y) = get_direction_offset::<f32>(&character.facing_direction);
            let frame_velocity = movement_data.velocity * delta_seconds;
            transform.prepend_translation_x(offset_x * frame_velocity);
            transform.prepend_translation_y(offset_y * frame_velocity);
        }

        for entity in static_characters {
            movements.remove(entity);
        }
    }
}
