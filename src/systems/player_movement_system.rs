use amethyst::{
    animation::{
        AnimationCommand,
        AnimationControlSet,
        ControlState,
    },
    ecs::{
        Join,
        ReadExpect,
        System,
        WriteStorage,
    },
    renderer::SpriteRender,
};

use crate::{
    entities::player::{
        Direction,
        Player,
        PlayerAction,
        PlayerAnimation,
        PlayerSpriteSheets,
    },
};

pub struct PlayerMovementSystem;

impl<'a> System<'a> for PlayerMovementSystem {
    type SystemData = (
        WriteStorage<'a, AnimationControlSet<PlayerAnimation, SpriteRender>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, PlayerSpriteSheets>,
    );

    fn run(&mut self, (
        mut control_sets,
        mut players,
        mut sprite_renders,
        sprite_sheets,
    ): Self::SystemData) {
        for (
            control_set,
            player,
            sprite_render,
        ) in (&mut control_sets, &mut players, &mut sprite_renders).join() {
            if player.temp_flag {
                match player.action {
                    PlayerAction::Walk => player_walk(sprite_render, &sprite_sheets),
                    PlayerAction::Run => player_run(sprite_render, &sprite_sheets),
                }

                let new_animation = get_new_animation(&player.action, &player.facing_direction);
                change_player_animation(&new_animation, control_set);
                player.temp_flag = false;
            }
        }
    }
}

pub fn get_new_animation(action: &PlayerAction, direction: &Direction) -> PlayerAnimation {
    match (action, direction) {
        (PlayerAction::Walk, Direction::Up) => PlayerAnimation::WalkUp,
        (PlayerAction::Walk, Direction::Down) => PlayerAnimation::WalkDown,
        (PlayerAction::Walk, Direction::Left) => PlayerAnimation::WalkLeft,
        (PlayerAction::Walk, Direction::Right) => PlayerAnimation::WalkRight,
        (PlayerAction::Run, Direction::Up) => PlayerAnimation::RunUp,
        (PlayerAction::Run, Direction::Down) => PlayerAnimation::RunDown,
        (PlayerAction::Run, Direction::Left) => PlayerAnimation::RunLeft,
        (PlayerAction::Run, Direction::Right) => PlayerAnimation::RunRight,
    }
}

pub fn player_walk(sprite_render: &mut SpriteRender, sprite_sheets: &PlayerSpriteSheets) {
    sprite_render.sprite_sheet = sprite_sheets.walking.clone();
    sprite_render.sprite_number = 0;
}

pub fn player_run(sprite_render: &mut SpriteRender, sprite_sheets: &PlayerSpriteSheets) {
    sprite_render.sprite_sheet = sprite_sheets.running.clone();
    sprite_render.sprite_number = 0;
}

fn change_player_animation(
    new_animation: &PlayerAnimation,
    control_set: &mut AnimationControlSet<PlayerAnimation, SpriteRender>,
) {
    control_set.animations
        .iter_mut()
        .filter(|(id, _)| *id != *new_animation)
        .for_each(|(_, animation)| {
            animation.command = AnimationCommand::Pause;
        });

    let (_, animation) = control_set.animations
        .iter_mut()
        .find(|(id, _)| *id == *new_animation)
        .unwrap();
    animation.state = ControlState::Requested;
    animation.command = AnimationCommand::Start;
}
