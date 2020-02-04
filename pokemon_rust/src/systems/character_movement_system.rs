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
    common::{Direction, get_direction_offset},
    entities::{
        AnimationTable,
        CharacterAnimation,
        character::{
            AllowedMovements,
            Character,
            CharacterMovement,
            MovementType,
            StepKind,
        },
        player::{
            PlayerAnimation,
            PlayerEntity,
        },
        npc::NpcAnimation,
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

                let new_animation = get_new_animation(is_player, &movement_data.movement_type, &character.facing_direction);
                animation_table.change_animation(new_animation);

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

                let new_animation = get_idle_animation(&character.facing_direction);
                animation_table.change_animation(new_animation);

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

pub fn get_new_animation(is_player: bool, movement_type: &MovementType, direction: &Direction) -> CharacterAnimation {
    match (is_player, movement_type, direction) {
        (true, MovementType::Walk, Direction::Up) => (PlayerAnimation::WalkUp).into(),
        (true, MovementType::Walk, Direction::Down) => (PlayerAnimation::WalkDown).into(),
        (true, MovementType::Walk, Direction::Left) => (PlayerAnimation::WalkLeft).into(),
        (true, MovementType::Walk, Direction::Right) => (PlayerAnimation::WalkRight).into(),
        (true, MovementType::Run, Direction::Up) => (PlayerAnimation::RunUp).into(),
        (true, MovementType::Run, Direction::Down) => (PlayerAnimation::RunDown).into(),
        (true, MovementType::Run, Direction::Left) => (PlayerAnimation::RunLeft).into(),
        (true, MovementType::Run, Direction::Right) => (PlayerAnimation::RunRight).into(),
        (false, _, Direction::Up) => (NpcAnimation::WalkUp).into(),
        (false, _, Direction::Down) => (NpcAnimation::WalkDown).into(),
        (false, _, Direction::Left) => (NpcAnimation::WalkLeft).into(),
        (false, _, Direction::Right) => (NpcAnimation::WalkRight).into(),
    }
}

pub fn get_idle_animation(direction: &Direction) -> CharacterAnimation {
    match direction {
        Direction::Up => CharacterAnimation::IdleUp,
        Direction::Down => CharacterAnimation::IdleDown,
        Direction::Left => CharacterAnimation::IdleLeft,
        Direction::Right => CharacterAnimation::IdleRight,
    }
}
