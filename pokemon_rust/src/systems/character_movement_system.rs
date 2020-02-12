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
            PlayerEntity,
            StepKind,
        },
    },
    events::EventQueue,
    map::{change_player_tile, CoordinateSystem, MapHandler},
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

        for (
            entity,
            character,
            movement_data,
            allowed_movements,
            transform,
            animation_table,
            sprite_render
        ) in (
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
                if map.is_tile_blocked(&movement_data.to) {
                    if is_player {
                        static_characters.push(entity);
                    }
                    continue;
                }

                on_movement_start(
                    character,
                    movement_data,
                    allowed_movements,
                    animation_table,
                    sprite_render,
                );

                map.mark_tile_as_solid(&movement_data.to);

                if !is_player {
                    map.change_npc_tile(&movement_data.from, &movement_data.to);
                }

                movement_data.started = true;
            }

            if movement_data.estimated_time <= delta_seconds {
                on_movement_finish(
                    character,
                    movement_data,
                    allowed_movements,
                    transform,
                    animation_table,
                    sprite_render,
                    if is_player { Some(&player_entity) } else { None },
                    &mut map,
                    &mut event_queue,
                );

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

fn on_movement_start(
    character: &Character,
    movement_data: &mut CharacterMovement,
    allowed_movements: &AllowedMovements,
    animation_table: &mut AnimationTable<CharacterAnimation>,
    sprite_render: &mut SpriteRender,
) {
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
}

fn on_movement_finish(
    character: &mut Character,
    movement_data: &mut CharacterMovement,
    allowed_movements: &AllowedMovements,
    transform: &mut Transform,
    animation_table: &mut AnimationTable<CharacterAnimation>,
    sprite_render: &mut SpriteRender,
    player_entity: Option<&PlayerEntity>,
    map: &mut MapHandler,
    event_queue: &mut EventQueue,
) {
    transform.set_translation(Vector3::new(
        movement_data.to.position.x(),
        movement_data.to.position.y(),
        0.,
    ));

    if let Some(player_entity) = player_entity {
        change_player_tile(
            &movement_data.from,
            &movement_data.to,
            &player_entity,
            map,
            event_queue,
        );
    }

    if movement_data.movement_type == MovementType::Run {
        let data = allowed_movements
            .get_movement_data(&MovementType::Walk)
            .unwrap();

        sprite_render.sprite_sheet = data.sprite_sheet.clone();
    }

    animation_table.change_animation(CharacterAnimation::Idle(
        character.facing_direction.clone(),
    ));

    character.next_step.invert();

    map.remove_solid_mark(&movement_data.from);
}
