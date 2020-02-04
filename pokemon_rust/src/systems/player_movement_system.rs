use amethyst::{
    core::{math::Vector3, Time, Transform},
    ecs::{
        Entities,
        Join,
        Read,
        ReadExpect,
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
            Character,
            CharacterMovement,
            MovementType,
            StepKind,
        },
        player::{
            PlayerAnimation,
            PlayerEntity,
            PlayerSpriteSheets,
        },
    },
    events::EventQueue,
    map::{change_tile, CoordinateSystem, MapHandler},
};

#[derive(Default)]
pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, Character>,
        WriteStorage<'a, CharacterMovement>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationTable<CharacterAnimation>>,
        WriteStorage<'a, SpriteRender>,
        Entities<'a>,
        ReadExpect<'a, PlayerSpriteSheets>,
        ReadExpect<'a, PlayerEntity>,
        WriteExpect<'a, MapHandler>,
        Write<'a, EventQueue>,
        Read<'a, Time>,
    );

    fn run(&mut self, (
        mut characters,
        mut movements,
        mut transforms,
        mut animation_tables,
        mut sprite_renders,
        entities,
        sprite_sheets,
        player_entity,
        mut map,
        mut event_queue,
        time,
    ): Self::SystemData) {
        let mut static_players = Vec::new();

        for (entity, character, movement_data, transform, animation_table, sprite_render) in (
            &entities,
            &mut characters,
            &mut movements,
            &mut transforms,
            &mut animation_tables,
            &mut sprite_renders,
        ).join() {
            let delta_seconds = time.delta_seconds();

            if !movement_data.started {
                if map.is_tile_blocked(&movement_data.to) {
                    static_players.push(entity);
                    continue;
                }

                match movement_data.movement_type {
                    // TODO: use the sprite sheet inside MovementData
                    MovementType::Walk => sprite_render.sprite_sheet = sprite_sheets.walking.clone(),
                    MovementType::Run => sprite_render.sprite_sheet = sprite_sheets.running.clone(),
                }

                let new_animation = get_new_animation(&movement_data.movement_type, &character.facing_direction);
                animation_table.change_animation(new_animation.into());

                if movement_data.step_kind == StepKind::Right {
                    animation_table.skip_to_frame_index(2);
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

                sprite_render.sprite_sheet = sprite_sheets.walking.clone();

                let new_animation = get_idle_animation(&character.facing_direction);
                animation_table.change_animation(new_animation.into());

                character.next_step.invert();

                static_players.push(entity);
                continue;
            }

            movement_data.estimated_time -= delta_seconds;

            let (offset_x, offset_y) = get_direction_offset::<f32>(&character.facing_direction);
            let frame_velocity = movement_data.velocity * delta_seconds;
            transform.prepend_translation_x(offset_x * frame_velocity);
            transform.prepend_translation_y(offset_y * frame_velocity);
        }

        for entity in static_players {
            movements.remove(entity);
        }
    }
}

pub fn get_new_animation(movement_type: &MovementType, direction: &Direction) -> PlayerAnimation {
    match (movement_type, direction) {
        (MovementType::Walk, Direction::Up) => PlayerAnimation::WalkUp,
        (MovementType::Walk, Direction::Down) => PlayerAnimation::WalkDown,
        (MovementType::Walk, Direction::Left) => PlayerAnimation::WalkLeft,
        (MovementType::Walk, Direction::Right) => PlayerAnimation::WalkRight,
        (MovementType::Run, Direction::Up) => PlayerAnimation::RunUp,
        (MovementType::Run, Direction::Down) => PlayerAnimation::RunDown,
        (MovementType::Run, Direction::Left) => PlayerAnimation::RunLeft,
        (MovementType::Run, Direction::Right) => PlayerAnimation::RunRight,
    }
}

pub fn get_idle_animation(direction: &Direction) -> PlayerAnimation {
    match direction {
        Direction::Up => PlayerAnimation::IdleUp,
        Direction::Down => PlayerAnimation::IdleDown,
        Direction::Left => PlayerAnimation::IdleLeft,
        Direction::Right => PlayerAnimation::IdleRight,
    }
}
