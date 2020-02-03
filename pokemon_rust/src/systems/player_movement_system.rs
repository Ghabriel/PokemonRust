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
            CharacterAction,
            CharacterMovement,
            StepKind,
        },
        player::{
            PlayerAction,
            PlayerAnimation,
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

                match movement_data.action {
                    CharacterAction::Player(PlayerAction::Walk) => sprite_render.sprite_sheet = sprite_sheets.walking.clone(),
                    CharacterAction::Player(PlayerAction::Run) => sprite_render.sprite_sheet = sprite_sheets.running.clone(),
                    _ => unreachable!(),
                }

                let new_animation = get_new_animation(&movement_data.action, &character.facing_direction);
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
                    &mut map,
                    &mut event_queue,
                );

                sprite_render.sprite_sheet = sprite_sheets.walking.clone();

                let new_animation = get_new_animation(&CharacterAction::Idle, &character.facing_direction);
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

pub fn get_new_animation(action: &CharacterAction, direction: &Direction) -> PlayerAnimation {
    match (action, direction) {
        (CharacterAction::Idle, Direction::Up) => PlayerAnimation::IdleUp,
        (CharacterAction::Idle, Direction::Down) => PlayerAnimation::IdleDown,
        (CharacterAction::Idle, Direction::Left) => PlayerAnimation::IdleLeft,
        (CharacterAction::Idle, Direction::Right) => PlayerAnimation::IdleRight,
        (CharacterAction::Player(PlayerAction::Walk), Direction::Up) => PlayerAnimation::WalkUp,
        (CharacterAction::Player(PlayerAction::Walk), Direction::Down) => PlayerAnimation::WalkDown,
        (CharacterAction::Player(PlayerAction::Walk), Direction::Left) => PlayerAnimation::WalkLeft,
        (CharacterAction::Player(PlayerAction::Walk), Direction::Right) => PlayerAnimation::WalkRight,
        (CharacterAction::Player(PlayerAction::Run), Direction::Up) => PlayerAnimation::RunUp,
        (CharacterAction::Player(PlayerAction::Run), Direction::Down) => PlayerAnimation::RunDown,
        (CharacterAction::Player(PlayerAction::Run), Direction::Left) => PlayerAnimation::RunLeft,
        (CharacterAction::Player(PlayerAction::Run), Direction::Right) => PlayerAnimation::RunRight,
        _ => unreachable!(),
    }
}
